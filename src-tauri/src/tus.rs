// Adapted version of https://github.com/zryambus/tus_async_client/ v0.2.1
// under The MIT License (MIT)

// Copyright (c) 2019 Jon Grythe Stødle

// Vendored in to avoid reqwest version conflicts
// and to be able to use rustls-tls instead of native-tls

//! # tus_async_client
//!
//! A Rust native client library to interact with *tus* enabled endpoints.
//!
//! ## Usage
//!
//! ```rust
//! use tus_async_client::Client;
//! use reqwest;
//! use std::rc::Rc;
//! use std::sync::Arc;
//!
//! // Create an instance of the `tus_async_client::Client` struct.
//! // Assumes "reqwest" feature is enabled (see above)
//! let client = Client::new(Arc::new(reqwest::Client::new()));
//!
//! // You'll need an upload URL to be able to upload a files.
//! // This may be provided to you (through a separate API, for example),
//! // or you might need to create the file through the *tus* protocol.
//! // If an upload URL is provided for you, you can skip this step.
//!
//! let upload_url = client
//! .create("https://my.tus.server/files/", "/path/to/file").await
//! .expect("Failed to create file on server");
//!
//! // Next, you can start uploading the file by calling `upload`.
//! // The file will be uploaded in 5 MiB chunks by default.
//! // To customize the chunk size, use `upload_with_chunk_size` instead of `upload`.
//!
//! client
//! .upload(&upload_url, "/path/to/file").await
//! .expect("Failed to upload file to server");
//! ```
//!
//! `upload` (and `upload_with_chunk_size`) will automatically resume the upload from where it left off, if the upload transfer is interrupted.

// use crate::http::{default_headers, Headers, HttpMethod, HttpRequest};
use crate::error::TusError;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use std::str::FromStr;

/// Indicates a byte offset withing a resource.
pub const UPLOAD_OFFSET: &str = "upload-offset";

/// Indicates the size of the entire upload in bytes.
pub const UPLOAD_LENGTH: &str = "upload-length";

/// The version of the protocol used by the client or the server.
pub const TUS_RESUMABLE: &str = "tus-resumable";

/// Use this header if its environment does not support the PATCH or DELETE methods.
pub const CONTENT_TYPE: &str = "content-type";

/// Use this header if its environment does not support the PATCH or DELETE methods.
pub const UPLOAD_METADATA: &str = "upload-metadata";

/// Use this header if its environment does not support the PATCH or DELETE methods.
pub const LOCATION: &str = "location";

const DEFAULT_CHUNK_SIZE: usize = 4 * 1024 * 1024;

/// Used to interact with a [tus](https://tus.io) endpoint.
pub struct TusClient {
    client: reqwest::Client,
}

impl TusClient {
    /// Instantiates a new instance of `Client`. `http_handler` needs to implement the `HttpHandler` trait.
    /// A default implementation of this trait for the `reqwest` library is available by enabling the `reqwest` feature.
    pub fn new(client: reqwest::Client) -> Self {
        TusClient { client }
    }

    fn create_request(&self, method: reqwest::Method, url: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, url)
            .header(TUS_RESUMABLE, "1.0.0")
    }

    /// Get info about a file on the server.
    pub async fn get_info(&self, url: &str) -> Result<UploadInfo, TusError> {
        let response = self
            .create_request(reqwest::Method::HEAD, url)
            .send()
            .await?;

        let bytes_uploaded = response
            .headers()
            .get(UPLOAD_OFFSET)
            .and_then(|l| l.to_str().ok()?.parse::<usize>().ok());
        let total_size = response
            .headers()
            .get(UPLOAD_LENGTH)
            .and_then(|l| l.to_str().ok()?.parse::<usize>().ok());

        if response.status().is_client_error() || bytes_uploaded.is_none() {
            return Err(TusError::NotFoundError);
        }

        let bytes_uploaded = bytes_uploaded.unwrap();

        Ok(UploadInfo {
            bytes_uploaded,
            total_size,
        })
    }

    /// Upload a file to the specified upload URL.
    pub async fn upload(&self, url: &str, path: &Path) -> Result<(), TusError> {
        self.upload_with_chunk_size(url, path, DEFAULT_CHUNK_SIZE)
            .await
    }

    /// Upload a file to the specified upload URL with the given chunk size.
    pub async fn upload_with_chunk_size(
        &self,
        url: &str,
        path: &Path,
        chunk_size: usize,
    ) -> Result<(), TusError> {
        let info = self.get_info(url).await?;
        let file = File::open(path)?;
        let file_len = file.metadata()?.len() as usize;

        if let Some(total_size) = info.total_size {
            if file_len != total_size {
                return Err(TusError::UnequalSizeError);
            }
        }

        let mut reader = BufReader::new(file);
        let mut buffer = vec![0; chunk_size];
        let mut progress = info.bytes_uploaded;

        reader.seek(SeekFrom::Start(progress as u64))?;

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                return Err(TusError::FileReadError);
            }

            let req = self
                .create_request(reqwest::Method::PATCH, url)
                .header(CONTENT_TYPE, "application/offset+octet-stream")
                .header(UPLOAD_OFFSET, progress.to_string())
                .body(buffer[..bytes_read].to_vec());

            let response = req.send().await?;

            if response.status().as_u16() == 409 {
                return Err(TusError::WrongUploadOffsetError);
            }

            if response.status().as_u16() == 404 {
                return Err(TusError::NotFoundError);
            }

            if response.status().as_u16() != 204 {
                return Err(TusError::UnexpectedStatusCode(response.status().as_u16()));
            }

            let upload_offset = match response.headers().get(UPLOAD_OFFSET) {
                Some(offset) => offset,
                None => return Err(TusError::MissingHeader(UPLOAD_OFFSET.to_owned())),
            };

            progress = match upload_offset.to_str()?.parse() {
                Ok(offset) => offset,
                Err(e) => return Err(TusError::ParsingError(e)),
            };

            if progress >= file_len {
                break;
            }
        }

        Ok(())
    }

    /// Create a file on the server including the specified metadata, receiving the upload URL of the file.
    pub async fn create_with_metadata(
        &self,
        url: &str,
        path: &Path,
        metadata: HashMap<String, String>,
    ) -> Result<String, TusError> {
        let mut req = self
            .create_request(reqwest::Method::POST, url)
            .header(UPLOAD_LENGTH.to_owned(), path.metadata()?.len().to_string());

        if !metadata.is_empty() {
            let data = metadata
                .iter()
                .map(|(key, value)| format!("{} {}", key, base64::encode(value)))
                .collect::<Vec<_>>()
                .join(",");
            req = req.header(UPLOAD_METADATA.to_owned(), data);
        }

        let response = req.send().await?;

        if response.status().as_u16() == 413 {
            return Err(TusError::FileTooLarge);
        }

        if response.status().as_u16() != 201 {
            return Err(TusError::UnexpectedStatusCode(response.status().as_u16()));
        }

        let location = response.headers().get(LOCATION);

        if location.is_none() {
            return Err(TusError::MissingHeader(LOCATION.to_owned()));
        }

        Ok(location.unwrap().to_str()?.to_owned())
    }
}

/// Describes a file on the server.
#[derive(Debug)]
pub struct UploadInfo {
    /// How many bytes have been uploaded.
    pub bytes_uploaded: usize,
    /// The total size of the file.
    pub total_size: Option<usize>,
}

/// Enumerates the extensions to the tus protocol.
#[derive(Debug, PartialEq)]
pub enum TusExtension {
    /// The server supports creating files.
    Creation,
    //// The server supports setting expiration time on files and uploads.
    Expiration,
    /// The server supports verifying checksums of uploaded chunks.
    Checksum,
    /// The server supports deleting files.
    Termination,
    /// The server supports parallel uploads of a single file.
    Concatenation,
}

impl FromStr for TusExtension {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "creation" => Ok(TusExtension::Creation),
            "expiration" => Ok(TusExtension::Expiration),
            "checksum" => Ok(TusExtension::Checksum),
            "termination" => Ok(TusExtension::Termination),
            "concatenation" => Ok(TusExtension::Concatenation),
            _ => Err(()),
        }
    }
}
