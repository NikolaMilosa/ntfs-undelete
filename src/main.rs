use clap::Parser;
use cli::Cli;
use dialoguer::MultiSelect;
use errors::Result;
use log::info;
use mft::MftParser;
use reader::Reader;

use crate::undelete_entry::UndeleteEntry;

mod cli;
mod errors;
mod reader;
mod undelete_entry;
mod util;

fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = cli::Cli::parse_and_validate(Cli::parse())?;
    args.display();

    let reader = Reader::from_path(args.image)?;

    let mft = reader.read_mft()?;
    info!("MFT size: {}", mft.len());

    let mut parser = MftParser::from_buffer(mft)?;

    let found = parser
        .iter_entries()
        .filter_map(|e| match e {
            Ok(entry) if !entry.is_dir() && !entry.is_allocated() => Some(entry.into()),
            _ => None,
        })
        .filter(|e: &UndeleteEntry| !e.filename.is_empty())
        .filter(|e| !e.is_allocated)
        .collect::<Vec<_>>();

    let found = found
        .iter()
        .map(|e| {
            let entry = parser.get_entry(e.record_number).unwrap();
            let full_path = match parser.get_full_path_for_entry(&entry) {
                Ok(Some(path)) => path.to_str().unwrap().to_string(),
                _ => e.filename.clone(),
            };

            UndeleteEntry {
                filename: full_path,
                record_number: e.record_number,
                is_allocated: e.is_allocated,
            }
        })
        .collect::<Vec<_>>();

    let chosen = MultiSelect::new()
        .with_prompt("Use UP and DOWN arrows to scroll up and down\nUse SPACE to select/unselect an option\nUse ENTER to finish\nChoose files to undelete")
        .max_length(10)
        .items(&found)
        .interact()?;

    if !args.dry_run && chosen.is_empty() {
        return Err(errors::Error::Any {
            detail: "No files selected".to_string(),
        });
    }

    let mut errors = vec![];

    for i in chosen {
        let undelete_entry = &found[i];
        let total_output_dir = args
            .output_dir
            .join(undelete_entry.filename.replace(['[', ']'], ""));

        if args.dry_run {
            info!("Would write to {}", total_output_dir.display());
            continue;
        }

        let entry = parser.get_entry(undelete_entry.record_number)?;

        if std::fs::metadata(total_output_dir.parent().unwrap()).is_err() {
            if let Err(e) = std::fs::create_dir_all(total_output_dir.parent().unwrap()) {
                errors.push(e);
                continue;
            };
        }

        if let Err(e) = std::fs::write(
            total_output_dir.as_path(),
            reader.read_data_from_entry(entry)?,
        ) {
            errors.push(e);
            continue;
        };
        info!("Successfully written to {}", total_output_dir.display());
    }

    if !errors.is_empty() {
        return Err(errors::Error::Any {
            detail: format!("{} errors occurred", errors.len()),
        });
    }

    Ok(())
}
