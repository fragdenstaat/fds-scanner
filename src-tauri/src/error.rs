use thiserror::Error;

#[derive(Error, Debug)]
#[error("{0}")]
pub struct AuthorizationError(pub String);

#[derive(Error, Debug)]
#[error("{0}")]
pub struct UserError(pub String);

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
    #[error("Framework error: {0}")]
    TauriError(#[from] tauri::Error),
    #[error("OAuth configuration error: {0}")]
    OAuthError(#[from] oauth2::ConfigurationError),
    #[error("Error storing data")]
    StoreError(#[from] tauri_plugin_store::Error),
    #[error("Could not scan document: {0}")]
    DocumentCamera(#[from] tauri_plugin_documentcamera::Error),
    #[error("Failed during upload: {0}")]
    TusError(#[from] tus_async_client::Error),
}
