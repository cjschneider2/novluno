use std::fmt::{Display, Formatter};
use std::io;
use std::str::Utf8Error;
use std::string::FromUtf16Error;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    FromUtf16(FromUtf16Error),
    FromUtf8(FromUtf8Error),
    Io(io::Error),
    MissingMapIdentifier,
    MissingRleIdentifier,
    UnknownOffsetTypeAt(u64, u8),
    Utf8(Utf8Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FromUtf16(s) => write!(f, "{}", s),
            Error::FromUtf8(s) => write!(f, "{}", s),
            Error::Io(s) => write!(f, "{}", s),
            Error::MissingMapIdentifier => write!(f, "{}", "missing map identifier"),
            Error::MissingRleIdentifier => write!(f, "{}", "missing rle identifier"),
            Error::UnknownOffsetTypeAt(offset, typ) => write!(f, "unknown type {} at offset {}", typ, offset),
            Error::Utf8(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::FromUtf16(_) => "error parsing utf16",
            Error::FromUtf8(_) => "error parsing utf8",
            Error::Io(_) => "io error",
            Error::MissingMapIdentifier => "missing map identifier",
            Error::MissingRleIdentifier => "missing rle identifier",
            Error::UnknownOffsetTypeAt(_, _) => "unknown type at offset",
            Error::Utf8(_) => "utf8 error",
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Error {
        Error::Utf8(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::FromUtf8(err)
    }
}

impl From<FromUtf16Error> for Error {
    fn from(err: FromUtf16Error) -> Error {
        Error::FromUtf16(err)
    }
}
