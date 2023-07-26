use clap::Parser;
use cli::Cli;
use errors::UndeleteError;
use parser_wrapper::ParserWrapper;

mod cli;
mod errors;
mod parser_wrapper;

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

    parser.print();
    Ok(())
}
