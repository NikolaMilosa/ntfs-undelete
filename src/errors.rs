use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("An I/O error has occurred")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("Failed to open file {}", path.display())]
    FailedToOpenFile {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("An unexpected error has occurred in mft: {}", detail)]
    Mft { detail: String },
    #[error("An unexpected error has occurred: {}", detail)]
    Any { detail: String },
}

impl From<mft::err::Error> for Error {
    fn from(source: mft::err::Error) -> Self {
        Self::Mft {
            detail: source.to_string(),
        }
    }
}
