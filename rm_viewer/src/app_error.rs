use std::io;
use std::str::Utf8Error;

use gtk;
use redmoon;

#[derive(Debug)]
pub enum AppError {
    Str(&'static str),
    StringConversion,
    Rm(redmoon::error::Error),
    Io(io::Error),
    Utf8(Utf8Error),
    Gtk(gtk::Error),
}

impl From<redmoon::error::Error> for AppError {
    fn from(err: redmoon::error::Error) -> AppError {
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
