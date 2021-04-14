use std::io;
use std::fmt;
use std::error;
use std::result;

use tera;
use serde_json as json;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Json(json::Error),
    Template(tera::Error),
    Parse(String),

    #[doc(hidden)]
    __Nonexhaustive
}

pub type Result<T> = result::Result<T, Error>;


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
            Error::Io(ref err) => {
                write!(f, "Io error: {}", err)
            }
            Error::Json(ref err) => {
                write!(f, "JSON error: {}", err)
            }
            Error::Template(ref err) => {
                write!(f, "Template engine error: {}", err)
            }
            Error::Parse(ref err) => {
                write!(f, "Service file parsing error: {}", err)
            }
            _ => unreachable!(),
		}
	}
}
