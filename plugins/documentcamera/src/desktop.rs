use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<DocumentCamera<R>> {
    Ok(DocumentCamera(app.clone()))
}

/// Access to the documentcamera APIs.
pub struct DocumentCamera<R: Runtime>(AppHandle<R>);

impl<R: Runtime> DocumentCamera<R> {
    pub fn scan(&self, _payload: ScanRequest) -> crate::Result<ScanResponse> {
        Ok(ScanResponse { path: None })
    }
}
