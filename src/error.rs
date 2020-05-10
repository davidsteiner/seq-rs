use crate::parser::Rule;
use pest::error::Error as PestError;

#[derive(Debug)]
pub enum Error {
    PestError(PestError<Rule>),
    ModelError { message: String },
}

impl From<PestError<Rule>> for Error {
    fn from(err: PestError<Rule>) -> Self {
        Error::PestError(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::PestError(err) => write!(f, "{}", err.to_string()),
            Error::ModelError { message } => write!(f, "{}", message),
        }
    }
}

impl Error {
    pub fn new(message: String) -> Error {
        Error::ModelError { message }
    }
}
