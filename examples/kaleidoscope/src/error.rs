use std::fmt::{self, Debug, Formatter};
use std::io;
use std::num::ParseFloatError;
use std::result;

use self::Error::*;

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
    FunctionRedef,
    FunctionRedefWithDifferentParams,
    Io(io::Error),
    LLVM(String),
    ParseFloat(ParseFloatError),
    Undefined(String),
    UnknownChar(char),
    Unexpected(&'static str),
    WrongArgumentCount,
}

impl Debug for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            FunctionRedef => write!(formatter, "redefinition of function"),
            FunctionRedefWithDifferentParams =>
                write!(formatter, "redefinition of function with different number of parameters"),
            Io(ref error) => error.fmt(formatter),
            LLVM(ref error) => write!(formatter, "LLVM error: {}", error),
            ParseFloat(ref error) => error.fmt(formatter),
            Undefined(ref msg) => write!(formatter, "undefined {}", msg),
            UnknownChar(char) => write!(formatter, "unknown char `{}`", char),
            Unexpected(msg) => write!(formatter, "unexpected {}", msg),
            WrongArgumentCount => write!(formatter, "wrong argument count"),
        }
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        LLVM(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Io(error)
    }
}

impl From<ParseFloatError> for Error {
    fn from(error: ParseFloatError) -> Self {
        ParseFloat(error)
    }
}
