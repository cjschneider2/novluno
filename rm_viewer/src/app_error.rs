use std::io;
use std::str::Utf8Error;

use gtk;
use core_compat;

#[derive(Debug)]
pub enum AppError {
    Str(&'static str),
    StringConversion,
    Rm(core_compat::error::Error),
    Io(io::Error),
    Utf8(Utf8Error),
    Gtk(gtk::Error),
}

impl From<core_compat::error::Error> for AppError {
    fn from(err: core_compat::error::Error) -> AppError {
        AppError::Rm(err)
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> AppError {
        AppError::Io(err)
    }
}

impl From<Utf8Error> for AppError {
    fn from(err: Utf8Error) -> AppError {
        AppError::Utf8(err)
    }
}

impl From<gtk::Error> for AppError {
    fn from(err: gtk::Error) -> AppError {
        AppError::Gtk(err)
    }
}
