use std::fmt;
use std::fmt::{Display, Formatter};
use std::io;
use std::process::exit;

#[derive(Copy, Clone)]
pub enum ExitCode {
  Ok = 0,
  ArgumentError,
  FailedToReadMaxBrightness,
  FailedToReadCurrentBrightness,
  FailedToSetBrightnesss,
  FailedToFindDevices,
}

pub enum Error {
  IOError(io::Error),
  FromUTF8(std::string::FromUtf8Error),
  NumberParseError(std::num::ParseIntError),
  Generic(String),
}

impl Error {
  pub fn fatal(self: &Self, exit_code: ExitCode) -> ! {
    println!("{}", self);
    exit(exit_code as i32);
  }
}

impl Display for Error {
  fn fmt(self: &Self, f: &mut Formatter<'_>) -> std::result::Result<(), fmt::Error> {
    match self {
      Error::IOError(e) => write!(f, "{}", e),
      Error::FromUTF8(e) => write!(f, "{}", e),
      Error::NumberParseError(e) => write!(f, "{}", e),
      Error::Generic(e) => write!(f, "{}", e),
    }
  }
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Error {
    Error::IOError(err)
  }
}

impl From<std::string::FromUtf8Error> for Error {
  fn from(err: std::string::FromUtf8Error) -> Error {
    Error::FromUTF8(err)
  }
}

impl From<std::num::ParseIntError> for Error {
  fn from(err: std::num::ParseIntError) -> Error {
    Error::NumberParseError(err)
  }
}

impl From<&str> for Error {
  fn from(text: &str) -> Error {
    Error::from(String::from(text))
  }
}

impl From<String> for Error {
  fn from(text: String) -> Error {
    Error::Generic(text)
  }
}

pub type Result<T> = std::result::Result<T, Error>;
