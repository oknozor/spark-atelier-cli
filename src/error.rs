use config::ConfigError as ConfError;
use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IOError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    ConfigError(ConfError),
    CommandError(IOError),
    FormatError(FromUtf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ConfigError(err) => write!(
                f,
                "Unable to read from config file :\n {}",
                err.description()
            ),
            Error::CommandError(err) => write!(f, "{}", err.description()),
            Error::FormatError(err) => write!(f, "{}", err.description()),
        }
    }
}
impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ConfigError(_) => "Process failed while reading config file",
            Error::CommandError(_) => "Process failed while executing command",
            Error::FormatError(_) => "Unable to read config file",
        }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error::FormatError(e)
    }
}

impl From<IOError> for Error {
    fn from(e: IOError) -> Self {
        Error::CommandError(e)
    }
}

impl From<ConfError> for Error {
    fn from(e: ConfError) -> Self {
        Error::ConfigError(e)
    }
}
