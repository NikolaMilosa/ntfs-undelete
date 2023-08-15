#[derive(Debug)]
pub enum UndeleteError {
    Parse(String),
    Initialization(String),
    Write(String),
    StdIO(String),
    EmptyChosenList(String),
}

impl From<std::io::Error> for UndeleteError {
    fn from(e: std::io::Error) -> Self {
        UndeleteError::StdIO(e.to_string())
    }
}
