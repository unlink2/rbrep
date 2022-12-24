use thiserror::Error;

pub type RbrepResult<T> = Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Syntax error")]
    BadSyntax,
    #[error("Unknown error")]
    Unknown,
}