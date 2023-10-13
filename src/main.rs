use std::io::Read;

use clap::Parser;
use cli::Cli;
use dialoguer::MultiSelect;
use errors::UndeleteError;
use log::info;
use tsk::bindings::TSK_FS_FILE_READ_FLAG_ENUM::TSK_FS_FILE_READ_FLAG_NONE;
use tsk::tsk_fs::{FSType, TskFs};
use tsk::tsk_fs_dir::TskFsDir;
use undelete_entry::UndeleteEntry;

mod cli;
mod errors;
mod undelete_entry;

fn main() -> Result<(), UndeleteError> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = cli::Cli::parse_and_validate(Cli::parse())?;
    args.display();

    let image_info = tsk::TskImg::from_utf8_sing(args.image.as_path()).map_err(|err| {
        UndeleteError::Initialization(format!("Failed to get image info: {:?}", err))
    })?;

    let tskfs = TskFs::from_fs_offset(&image_info, 0)
        .map_err(|err| UndeleteError::Parse(format!("Couldn't parse image file: {:?}", err)))?;

    match tskfs.fs_type() {
        FSType::NTFS => info!("Running: {:?}", tskfs.fs_type()),
        _ => return Err(UndeleteError::UnsupportedFs(tskfs.fs_type())),
    };

    let unallocated = get_all_unallocated_files_from_dir("/", &tskfs)?;

    let chosen = MultiSelect::new()
        .with_prompt("Use UP and DOWN arrows to scroll up and down\nUse SPACE to select/unselect an option\nUse ENTER to finish\nChoose files to undelete")
        .max_length(10)
        .items(&unallocated)
        .interact()?;

    for file in chosen {
        let entry = unallocated.get(file).unwrap();
        let full_path_relative_to_img = entry.get_full_path();
        let new_path = args.output_dir.join(&full_path_relative_to_img);

        let file = tskfs.file_open_meta(entry.inode).map_err(|err| {
            UndeleteError::General(format!("Couldn't open file {}: {:?}", entry.name, err))
        })?;

        let mut handle = file
            .get_file_handle(file.get_attr().unwrap(), TSK_FS_FILE_READ_FLAG_NONE)
            .map_err(|err| {
                UndeleteError::StdIO(format!(
                    "Coudln't get file handle for file {}: {:?}",
                    entry.name, err
                ))
            })?;

        let file_size = file
            .get_meta()
            .map_err(|err| {
                UndeleteError::General(format!(
                    "Couldn't get file meta for file {}: {:?}",
                    entry.name, err
                ))
            })?
            .size();

        let mut buf: Vec<u8> = Vec::with_capacity(file_size.try_into().unwrap());

        handle.read_to_end(&mut buf).map_err(|err| {
            UndeleteError::StdIO(format!(
                "Couldn't read into buffer for file {}: {:?}",
                entry.name, err
            ))
        })?;

        if args.dry_run {
            info!("Would write '{}' to disk", new_path.display());
            continue;
        }

        if std::fs::metadata(new_path.parent().unwrap()).is_err() {
            std::fs::create_dir_all(new_path.parent().unwrap()).map_err(|err| {
                UndeleteError::General(format!(
                    "Couldn't create parent directories for full path '{}': {:?}",
                    new_path.display(),
                    err,
                ))
            })?;
        }

        match std::fs::write(&new_path, buf) {
            Ok(_) => info!("Successfully wrote '{}' to disk", new_path.display()),
            Err(e) => return Err(UndeleteError::Write(e.to_string())),
        }
    }

    Ok(())
}

fn get_all_unallocated_files_from_dir(
    dir: &str,
    tskfs: &TskFs,
) -> Result<Vec<UndeleteEntry>, UndeleteError> {
    let mut current_entries = vec![];

    info!("Running getting unallocated for {}", dir);

    for item in TskFsDir::from_path(tskfs, dir)
        .map_err(|err| {
            UndeleteError::General(format!(
                "Recursive getting files failed for path '{}': {:?}",
                dir, err
            ))
        })?
        .get_name_iter()
    {
        if item.is_dir()
            && item.name().unwrap() != "."
            && item.name().unwrap() != ".."
            && !item.name().unwrap().starts_with('$')
        {
            current_entries.extend(get_all_unallocated_files_from_dir(
                &format!(
                    "{}/{}",
                    match dir {
                        "/" => "",
                        rest => rest,
                    },
                    &item.name().unwrap()
                ),
                tskfs,
            )?);
            continue;
        }

        if item.is_allocated() || item.name().unwrap() == "." || item.name().unwrap() == ".." {
            continue;
        }

        current_entries.push(UndeleteEntry {
            name: item.name().unwrap(),
            inode: item.get_inode(),
            dir: dir.to_string(),
        })
    }

    Ok(current_entries)
}
