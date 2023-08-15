use clap::Parser;
use cli::Cli;
use dialoguer::MultiSelect;
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

    let entries = parser.get_all_entries();
    let filtered_entries = entries
        .iter()
        .filter(|e| !e.is_allocated)
        .collect::<Vec<_>>();

    if filtered_entries.is_empty() {
        info!("No files to undelete");
        return Ok(());
    }

    let chosen = MultiSelect::new()
        .with_prompt("Use UP and DOWN arrows to scroll up and down\nUse SPACE to select/unselect an option\nUse ENTER to finish\nChoose files to undelete")
        .max_length(10)
        .items(&filtered_entries)
        .interact()?;

    if chosen.is_empty() && !args.dry_run {
        return Err(UndeleteError::EmptyChosenList(
            "No files were chosen".to_string(),
        ));
    }

    for i in chosen {
        let entry = filtered_entries[i];
        let path = args.output_dir.join(entry.filename.clone());

        info!("Undeleting file: {}", entry.filename);
        if !args.dry_run {
            match std::fs::write(path.clone(), &entry.data) {
                Ok(_) => info!("Successfully wrote '{}' to disk", path.display()),
                Err(e) => return Err(UndeleteError::Write(e.to_string())),
            }
        }
    }

    Ok(())
}
