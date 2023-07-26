#[derive(Debug)]
pub enum UndeleteError {
    ParseError(String),
    InitializationError(String),
}
