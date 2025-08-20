use std::error::Error as StdError;
use std::{
    fmt::{Display, Formatter},
    io,
    num::ParseIntError,
};

use thiserror::Error;

#[derive(Error, Debug)]
#[error("{0}")]
pub struct AuthorizationError(pub String);

#[derive(Error, Debug)]
#[error("{0}")]
pub struct UserError(pub String);

/// Enumerates the errors which can occur during operation
#[derive(Debug)]
pub enum TusError {
    /// The status code returned by the server was not one of the expected ones.
    UnexpectedStatusCode(u16),
    /// The file specified was not found by the server.
    NotFoundError,
    /// A required header was missing from the server response.
    MissingHeader(String),
    /// An error occurred while doing disk IO. This may be while reading a file, or during a network call.
    IoError(io::Error),
    /// Unable to parse a value, which should be an integer.
    ParsingError(ParseIntError),
    HeaderParsingError(reqwest::header::ToStrError),
    /// The size of the specified file, and the file size reported by the server do not match.
    UnequalSizeError,
    /// Unable to read the file specified.
    FileReadError,
    /// The `Client` tried to upload the file with an incorrect offset.
    WrongUploadOffsetError,
    /// The specified file is larger that what is supported by the server.
    FileTooLarge,
    RequestError(reqwest::Error),
}

impl Display for TusError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let message = match self {
            TusError::UnexpectedStatusCode(status_code) => format!("The status code returned by the server was not one of the expected ones: {status_code}"),
            TusError::NotFoundError => "The file specified was not found by the server".to_string(),
            TusError::MissingHeader(header_name) => format!("The '{header_name}' header was missing from the server response"),
            TusError::IoError(error) => format!("An error occurred while doing disk IO. This may be while reading a file, or during a network call: {error}"),
            TusError::ParsingError(error) => format!("Unable to parse a value, which should be an integer: {error}"),
            TusError::UnequalSizeError => "The size of the specified file, and the file size reported by the server do not match".to_string(),
            TusError::FileReadError => "Unable to read the specified file".to_string(),
            TusError::WrongUploadOffsetError => "The client tried to upload the file with an incorrect offset".to_string(),
            TusError::FileTooLarge => "The specified file is larger that what is supported by the server".to_string(),
            TusError::RequestError(error) => format!("Error during HTTP request: {error}"),
            TusError::HeaderParsingError(error) => format!("Could not convert header to str: {error}"),
        };

        write!(f, "{message}")?;

        Ok(())
    }
}

impl StdError for TusError {}

impl From<io::Error> for TusError {
    fn from(e: io::Error) -> Self {
        TusError::IoError(e)
    }
}

impl From<ParseIntError> for TusError {
    fn from(e: ParseIntError) -> Self {
        TusError::ParsingError(e)
    }
}

impl From<reqwest::Error> for TusError {
    fn from(e: reqwest::Error) -> Self {
        TusError::RequestError(e)
    }
}

impl From<reqwest::header::ToStrError> for TusError {
    fn from(e: reqwest::header::ToStrError) -> Self {
        TusError::HeaderParsingError(e)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Failed to parse URL: {0}")]
    UrlParse(#[from] oauth2::url::ParseError),
    #[error("Failed to perform authentication: {0}")]
    WebAuthError(#[from] tauri_plugin_webauth::Error),
    #[error("Authorization error: {0}")]
    AuthorizationError(#[from] AuthorizationError),
    #[error("User error: {0}")]
    UserError(#[from] UserError),
    #[error("Failed to perform request: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Could not convert header to str: {0}")]
    HeaderResponseError(#[from] reqwest::header::ToStrError),
    #[error("Framework error: {0}")]
    TauriError(#[from] tauri::Error),
    #[error("OAuth configuration error: {0}")]
    OAuthError(#[from] oauth2::ConfigurationError),
    #[error("Error storing data")]
    StoreError(#[from] tauri_plugin_store::Error),
    #[error("Could not scan document: {0}")]
    DocumentCamera(#[from] tauri_plugin_documentcamera::Error),
    #[error("Problem with scan: {0}")]
    DocumentCameraResult(String),
    #[error("Failed during upload: {0}")]
    TusError(#[from] TusError),
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
}
