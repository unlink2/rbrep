use thiserror::Error;

pub type RbrepResult<T> = Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Syntax error")]
    BadSyntax(usize),
    #[error("Unknown error")]
    EndOfFile,
    #[error("Unknown error")]
    Io,
    #[error("Unknown error")]
    Unknown,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
