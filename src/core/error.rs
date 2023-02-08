use thiserror::Error;

pub type RbrepResult<T> = Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Syntax error")]
    BadSyntax(usize),
    #[error("EndOfFile")]
    EndOfFile,
    #[error("IO error")]
    Io,
    #[error("Unknown error")]
    Unknown,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
