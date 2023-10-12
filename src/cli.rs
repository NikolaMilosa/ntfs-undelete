use std::path::PathBuf;

use clap::{arg, Parser};
use log::info;

use crate::errors::UndeleteError;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(
        long,
        short,
        default_value = "false",
        help = r#"
Outputs the contents of MFT found in the root of the volume and calculates what will be restored.
"#
    )]
    dry_run: bool,

    #[arg(
        long,
        short,
        help = r#"
The path to which the restored files will be written.
"#
    )]
    output_dir: PathBuf,

    #[arg(
        long,
        short,
        help = r#"
The path to the image of file system to be restored that was retrieved with dd.
"#
    )]
    image: PathBuf,
}

pub struct CliParsed {
    pub dry_run: bool,
    pub output_dir: PathBuf,
    pub image: PathBuf,
}

impl CliParsed {
    pub fn display(&self) {
        info!("Configured with dry-run: {}", self.dry_run);
        info!("Configured with output-dir: {}", self.output_dir.display());
        info!("Configured with volume: {}", self.image.display());
    }

    pub fn parse_and_validate(args: Cli) -> Result<Self, UndeleteError> {
        if !args.image.exists() {
            return Err(UndeleteError::Parse(
                "Specified volume is Non-existant!".to_string(),
            ));
        }

        Ok(args.into())
    }
}

impl From<Cli> for CliParsed {
    fn from(value: Cli) -> Self {
        CliParsed {
            dry_run: value.dry_run,
            output_dir: value.output_dir,
            image: value.image,
        }
    }
}
