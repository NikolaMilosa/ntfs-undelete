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
The path to the volume to be restored. Conflicts with the image option.
"#
    )]
    volume: Option<PathBuf>,

    #[arg(
        long,
        short,
        help = r#"
The path to the image of mft file to be restored. Conflicts with the volume option.
"#
    )]
    image: Option<PathBuf>,
}

pub struct CliParsed {
    pub dry_run: bool,
    pub output_dir: PathBuf,
    pub volume: PathBuf,
}

impl CliParsed {
    pub fn display(&self) {
        info!("Configured with dry-run: {}", self.dry_run);
        info!("Configured with output-dir: {}", self.output_dir.display());
        info!("Configured with volume: {}", self.volume.display());
    }

    pub fn parse_and_validate(args: Cli) -> Result<Self, UndeleteError> {
        if args.volume.is_none() && args.image.is_none() {
            return Err(UndeleteError::Parse(
                "Either volume or image must be specified".to_string(),
            ));
        }
        if args.volume.is_some() && args.image.is_some() {
            return Err(UndeleteError::Parse(
                "Only one of volume or image must be specified".to_string(),
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
            volume: value.image.unwrap_or_else(|| {
                let mut v = value.volume.unwrap();
                v.push("$MFT");
                v
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_validate_no_volume_no_image() {
        let non_parsed = Cli {
            dry_run: false,
            output_dir: PathBuf::from("/tmp"),
            volume: None,
            image: None,
        };

        let args = CliParsed::parse_and_validate(non_parsed);
        assert!(args.is_err());
    }

    #[test]
    fn test_parse_and_validate_both_volume_and_image() {
        let non_parsed = Cli {
            dry_run: false,
            output_dir: PathBuf::from("/tmp"),
            volume: Some(PathBuf::from("/tmp")),
            image: Some(PathBuf::from("/tmp")),
        };

        let args = CliParsed::parse_and_validate(non_parsed);
        assert!(args.is_err());
    }

    #[test]
    fn test_parse_and_validate_only_volume() {
        let non_parsed = Cli {
            dry_run: false,
            output_dir: PathBuf::from("/tmp"),
            volume: Some(PathBuf::from("/tmp")),
            image: None,
        };

        let args = CliParsed::parse_and_validate(non_parsed);
        assert!(args.is_ok());
    }

    #[test]
    fn test_parse_and_validate_only_image() {
        let non_parsed = Cli {
            dry_run: false,
            output_dir: PathBuf::from("/tmp"),
            volume: None,
            image: Some(PathBuf::from("/tmp")),
        };

        let args = CliParsed::parse_and_validate(non_parsed);
        assert!(args.is_ok());
    }
}
