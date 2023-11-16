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
            Ok(entry) => Some(entry.into()),
            Err(_) => None,
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

    for i in chosen {
        let undelete_entry = &found[i];
        info!(
            "Undeleting {} with record number {}",
            undelete_entry.filename, undelete_entry.record_number
        );

        let entry = parser.get_entry(undelete_entry.record_number)?;
        info!(
            "Writing to {}",
            args.output_dir
                .join(undelete_entry.filename.as_str())
                .display()
        );
    }

    Ok(())
}
