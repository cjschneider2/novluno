use std::io;
use std::str::Utf8Error;

use core_compat;

#[derive(Debug)]
pub enum Error {
    SpriteLoad,
    Rm(core_compat::error::Error),
    Io(io::Error),
    Utf8(Utf8Error),
}

impl From<core_compat::error::Error> for Error {
    fn from(err: core_compat::error::Error) -> Error {
        Error::Rm(err)
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
