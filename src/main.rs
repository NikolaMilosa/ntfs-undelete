use std::path::PathBuf;

use clap::Parser;
use cli::Cli;
use errors::UndeleteError;
use log::info;
use parser_wrapper::ParserWrapper;

mod cli;
mod errors;
mod parser_wrapper;
mod undelete_entry;

fn main() -> Result<(), UndeleteError> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = cli::CliParsed::parse_and_validate(Cli::parse())?;
    args.display();

    let mut parser = match ParserWrapper::new(args.volume) {
        Ok(p) => p,
        Err(e) => {
            return Err(e);
        }
    };

    let message = match args.dry_run {
        true => "Dry run mode enabled, no files will be written to disk",
        false => "Dry run mode disabled, files will be written to disk",
    };
    info!("{}", message);

    for entry in parser.get_all_entries().iter().filter(|e| !e.is_allocated) {
        let calculated_path = PathBuf::from(&args.output_dir).join(&entry.filename);
        info!(
            "Found deleted entry '{}' with {}B of data, will write it to {}",
            entry.filename,
            entry.data.len(),
            calculated_path.display()
        );
        if !args.dry_run {
            match std::fs::write(calculated_path.clone(), &entry.data) {
                Ok(_) => info!("Successfully wrote '{}' to disk", calculated_path.display()),
                Err(e) => return Err(UndeleteError::Write(e.to_string())),
            }
        }
    }

    Ok(())
}
