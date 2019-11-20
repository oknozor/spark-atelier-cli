use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IOError;
use std::string::FromUtf8Error;
use config::ConfigError as ConfError;

#[derive(Debug)]
pub enum Error {
    ConfigError,
    CommandError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ConfigError => f.write_str("Unable to read from config file"),
            Error::CommandError => write!(f, "Command failed"),
        }
    }
}
impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ConfigError => ("Process failed while reading config file"),
            Error::CommandError => ("Process failed while executing command"),
        }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error::ConfigError
    }
}

impl From<IOError> for Error {
    fn from(e: IOError) -> Self {
        Error::ConfigError
    }
}


impl From<ConfError> for Error {
    fn from(e: ConfError) -> Self {
        Error::ConfigError
    }
}
