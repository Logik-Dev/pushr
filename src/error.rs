use std::path::PathBuf;

use derive_more::derive::From;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // -- Cli
    FileNotFound(PathBuf),
    FileIsEmpty(PathBuf),

    // -- External
    #[from]
    Io(std::io::Error),
    #[from]
    Request(reqwest::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
