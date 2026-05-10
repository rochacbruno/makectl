use thiserror::Error;

#[derive(Error, Debug)]
pub enum MakectlError {
    #[error("Parse error at line {line}: {message}")]
    ParseError { line: usize, message: String },

    #[error("No managed block found for target '{name}'")]
    ManagedBlockNotFound { name: String },

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
