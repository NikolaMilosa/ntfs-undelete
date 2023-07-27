#[derive(Debug)]
pub enum UndeleteError {
    Parse(String),
    Initialization(String),
    Write(String),
}
