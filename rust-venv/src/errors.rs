use std::fmt;
use std::io;

#[derive(Debug)]
pub enum CommandError {
    IoError {
        source: io::Error,
    },
    NonZeroExit {
        code: i32,
    },
    NoExitCode {},
    NoVenv {},
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandError::IoError { source } => {
                write!(f, "I/O Error: {}", source)
            },
            CommandError::NonZeroExit { code } => {
                write!(f, "Command execution failed with non-zero exit status: {}", code)
            },
            CommandError::NoExitCode {} => {
                write!(f, "Command execution failed.")
            },
            CommandError::NoVenv {} => {
                write!(f, "Virtual environment not found.")
            }
        }
    }
}

impl std::error::Error for CommandError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CommandError::IoError { source } => Some(source),
            _ => None,
        }
    }
}