use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChessError {
    // wrap std::io::Error, automatically implement `From<io::Error>`:
    #[error("I/O failure: {0}")]
    Io(#[from] std::io::Error),

    // wrap parse int errors:
    #[error("ParseInt failed: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    // a pure‐custom error with its own message
    #[error("Configuration missing field: {0}")]
    MissingField(String),
}
