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

    for entry in parser.get_all_entry_names() {
        info!("Entry: {:?}", entry);
    }
    Ok(())
}
