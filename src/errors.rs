use tsk::tsk_fs::FSType;

#[derive(Debug)]
pub enum UndeleteError {
    Parse(String),
    Initialization(String),
    UnsupportedFs(FSType),
    General(String),
    StdIO(String),
    Write(String),
}

impl From<std::io::Error> for UndeleteError {
    fn from(e: std::io::Error) -> Self {
        UndeleteError::StdIO(e.to_string())
    }
}
