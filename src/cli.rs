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
    pub dry_run: bool,

    #[arg(
        long,
        short,
        help = r#"
The path to which the restored files will be written.
"#
    )]
    pub output_dir: PathBuf,

    #[arg(
        long,
        short,
        help = r#"
The path to the image of file system to be restored that was retrieved with dd.
"#
    )]
    pub image: PathBuf,
}

impl Cli {
    pub fn display(&self) {
        info!("Configured with dry-run: {}", self.dry_run);
        info!("Configured with output-dir: {}", self.output_dir.display());
        info!("Configured with image: {}", self.image.display());
    }

    pub fn parse_and_validate(self) -> Result<Self, UndeleteError> {
        if !self.image.exists() {
            return Err(UndeleteError::Parse(
                "Specified image is Non-existant!".to_string(),
            ));
        }

        if !self.output_dir.exists() && !self.dry_run {
            return Err(UndeleteError::Parse(
                "Specified output directory is Non-existant!".to_string(),
            ));
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_validate_non_existant_image() {
        let args = Cli::parse_and_validate(Cli {
            dry_run: false,
            image: PathBuf::from("non-existant/ntfs.dd"),
            output_dir: PathBuf::from("src"),
        });
        assert!(args.is_err());
    }
}
