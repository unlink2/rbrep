use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Syntax error")]
    BadSyntax,
}
