use thiserror::Error;
use std::io;
use serde_json;
use url;
use reqwest;
use did_url;
use did_resolver;
use dwn_sdk;

#[derive(Error, Debug)]
pub enum Web5Error {
    #[error("Web5 operation failed: {0}")]
    OperationFailed(String),
    
    #[error("Invalid DID: {0}")]
    InvalidDID(String),
    
    #[error("Record not found")]
    RecordNotFound,
    
    #[error("Authentication failed")]
    AuthenticationFailed,
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    
    #[error("Decryption error: {0}")]
    DecryptionError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
    
    #[error("Resource not available: {0}")]
    ResourceNotAvailable(String),
    
    #[error("Timeout error")]
    TimeoutError,
    
    #[error("DID resolution error: {0}")]
    DIDResolutionError(String),
    
    #[error("DWN error: {0}")]
    DWNError(String),
    
    #[error("URL parsing error: {0}")]
    URLParsingError(String),
    
    #[error("HTTP client error: {0}")]
    HTTPClientError(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<io::Error> for Web5Error {
    fn from(error: io::Error) -> Self {
        Web5Error::OperationFailed(error.to_string())
    }
}

impl From<serde_json::Error> for Web5Error {
    fn from(error: serde_json::Error) -> Self {
        Web5Error::SerializationError(error.to_string())
    }
}

impl From<url::ParseError> for Web5Error {
    fn from(error: url::ParseError) -> Self {
        Web5Error::URLParsingError(error.to_string())
    }
}

impl From<reqwest::Error> for Web5Error {
    fn from(error: reqwest::Error) -> Self {
        Web5Error::HTTPClientError(error.to_string())
    }
}

impl From<did_url::DIDUrlError> for Web5Error {
    fn from(error: did_url::DIDUrlError) -> Self {
        Web5Error::InvalidDID(error.to_string())
    }
}

impl From<did_resolver::Error> for Web5Error {
    fn from(error: did_resolver::Error) -> Self {
        Web5Error::DIDResolutionError(error.to_string())
    }
}

impl From<dwn_sdk::Error> for Web5Error {
    fn from(error: dwn_sdk::Error) -> Self {
        Web5Error::DWNError(error.to_string())
    }
}
