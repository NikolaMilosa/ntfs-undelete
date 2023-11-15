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

    // let mut i = 0;
    // let jump = 1024;
    // loop {
    //     for j in i * jump..(i + 1) * jump {
    //         print!("{:02x} ", mft[j]);
    //     }
    //     println!();
    //     i += 1;
    //     if i * jump >= mft.len() {
    //         break;
    //     }
    // }

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

    let chosen = MultiSelect::new()
        .with_prompt("Use UP and DOWN arrows to scroll up and down\nUse SPACE to select/unselect an option\nUse ENTER to finish\nChoose files to undelete")
        .max_length(10)
        .items(&found)
        .interact()?;

    info!("Chosen entries: {:?}", chosen);

    Ok(())
}
