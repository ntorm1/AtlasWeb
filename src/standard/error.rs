use std::io;
use std::num::{ParseFloatError, ParseIntError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AtlasError {
    #[error("{file}:{line} - Assumption error: {msg}")]
    AssumptionError {
        msg: String,
        file: &'static str,
        line: u32,
    },

    #[error("{file}:{line} - IO error: {msg}")]
    IOError {
        msg: String,
        file: &'static str,
        line: u32,
    },

    #[error("{file}:{line} - ParseError: {msg}")]
    ParseError {
        msg: String,
        file: &'static str,
        line: u32,
    },

    #[error("{file}:{line} - CSV error: {msg}")]
    CSVError {
        msg: String,
        file: &'static str,
        line: u32,
    },
}

impl From<io::Error> for AtlasError {
    fn from(error: io::Error) -> Self {
        AtlasError::IOError {
            msg: error.to_string(),
            file: file!(),
            line: line!(),
        }
    }
}

impl From<csv::Error> for AtlasError {
    fn from(error: csv::Error) -> Self {
        AtlasError::CSVError {
            msg: error.to_string(),
            file: file!(),
            line: line!(),
        }
    }
}

impl From<chrono::format::ParseError> for AtlasError {
    fn from(error: chrono::format::ParseError) -> Self {
        AtlasError::ParseError {
            msg: error.to_string(),
            file: file!(),
            line: line!(),
        }
    }
}

impl From<ParseIntError> for AtlasError {
    fn from(err: ParseIntError) -> Self {
        AtlasError::ParseError {
            msg: err.to_string(),
            file: "unknown",
            line: 0,
        }
    }
}

impl From<ParseFloatError> for AtlasError {
    fn from(err: ParseFloatError) -> Self {
        AtlasError::ParseError {
            msg: err.to_string(),
            file: "unknown",
            line: 0,
        }
    }
}

#[macro_export]
macro_rules! 
assumption_error {
    ($msg:expr) => {
        Err(AtlasError::AssumptionError {
            msg: $msg.to_string(),
            file: file!(),
            line: line!(),
        })
    };
    ($msg:expr, $source:expr) => {
        Err(AtlasError::AssumptionError {
            msg: format!("{}: {}", $msg, $source),
            file: file!(),
            line: line!(),
        })
    };
}
