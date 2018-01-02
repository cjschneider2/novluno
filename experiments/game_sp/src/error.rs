use std::io;
use std::str::Utf8Error;

use sdl2::IntegerOrSdlError;
use sdl2::video::WindowBuildError;

use core_compat;

#[derive(Debug)]
pub enum Error {
    SpriteLoad,
    MapLoad,
    Rm(core_compat::error::Error),
    Io(io::Error),
    Utf8(Utf8Error),
    Str(String),
    WindowBuildError,
    IntegerOrSdlError,
}

impl From<core_compat::error::Error> for Error {
    fn from(err: core_compat::error::Error) -> Error {
        Error::Rm(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Str(err)
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

impl From<WindowBuildError> for Error {
    fn from(_: WindowBuildError) -> Error {
        Error::WindowBuildError
    }
}

impl From<IntegerOrSdlError> for Error {
    fn from(_: IntegerOrSdlError) -> Error {
        Error::IntegerOrSdlError
    }
}
