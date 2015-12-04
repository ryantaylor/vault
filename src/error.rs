//! A module containing error information related to the vault parser.

use std::convert::From;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;
use std::io::Error as IoError;
use std::string::{FromUtf8Error, FromUtf16Error};

#[cfg(feature = "parse-archive")]
use zip::result::ZipError;

/// This type contains the various error messages that can be returned from the library.

#[derive(Debug)]
pub enum Error {
    CursorWrap,
    CursorOutOfBounds,
    FileTooLarge,
    EmptyChar,
    InvalidFileExtension,
    UnexpectedValue,
    UnsupportedVersion,
    UnsupportedChunkVersion,
    IoError(IoError),
    Utf8Error(FromUtf8Error),
    Utf16Error(FromUtf16Error),
    #[cfg(feature = "parse-archive")]
    ZipError(ZipError),
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::IoError(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::Utf8Error(err)
    }
}

impl From<FromUtf16Error> for Error {
    fn from(err: FromUtf16Error) -> Error {
        Error::Utf16Error(err)
    }
}

#[cfg(feature = "parse-archive")]
impl From<ZipError> for Error {
    fn from(err: ZipError) -> Error {
        Error::ZipError(err)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::CursorWrap => "Operation would cause cursor to wrap around integer bounds.",
            Error::CursorOutOfBounds => "Attempting to read at out-of-bounds cursor.",
            Error::FileTooLarge => "Cannot load file, too large.",
            Error::EmptyChar => "Empty character read.",
            Error::InvalidFileExtension => "Cannot parse the given filetype.",
            Error::UnexpectedValue => "Unexpected value found during parsing.",
            Error::UnsupportedVersion => "Version must be 19545 (UKF release) or higher.",
            Error::UnsupportedChunkVersion => "Encountered a chunk version that could not be parsed. Please check for an update.",
            Error::IoError(ref err) => err.description(),
            Error::Utf8Error(ref err) => err.description(),
            Error::Utf16Error(ref err) => err.description(),
            #[cfg(feature = "parse-archive")]
            Error::ZipError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        Some(match *self {
            Error::IoError(ref err) => err as &StdError,
            Error::Utf8Error(ref err) => err as &StdError,
            Error::Utf16Error(ref err) => err as &StdError,
            #[cfg(feature = "parse-archive")]
            Error::ZipError(ref err) => err as &StdError,
            _ => self as &StdError,
        })
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            Error::CursorWrap => fmt.write_str("Buffer read error (CursorWrap)"),
            Error::CursorOutOfBounds => fmt.write_str("Buffer read error (CursorOutOfBounds)"),
            Error::FileTooLarge => fmt.write_str("The file is too large (FileTooLarge)"),
            Error::EmptyChar => fmt.write_str("Buffer read error (EmptyChar)"),
            Error::InvalidFileExtension => fmt.write_str("The filetype is invalid (InvalidFileExtension)"),
            Error::UnexpectedValue => fmt.write_str("An unexpected value was encountered (UnexpectedValue)"),
            Error::UnsupportedVersion => fmt.write_str("An unsupported version was encountered (UnsupportedVersion)"),
            Error::UnsupportedChunkVersion => fmt.write_str("An unsupported chunk version was encountered (UnsupportedChunkVersion)"),
            Error::IoError(ref err) => Display::fmt(err, fmt),
            Error::Utf8Error(ref err) => Display::fmt(err, fmt),
            Error::Utf16Error(ref err) => Display::fmt(err, fmt),
            #[cfg(feature = "parse-archive")]
            Error::ZipError(ref err) => Display::fmt(err, fmt),
        }
    }
}