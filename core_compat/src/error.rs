use std::io;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    MissingRleIdentifier,
    MissingMapIdentifier,
    UnknownOffsetTypeAt(u64),
    Io(io::Error),
    Utf8(Utf8Error),
    FromUtf8(FromUtf8Error),
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
