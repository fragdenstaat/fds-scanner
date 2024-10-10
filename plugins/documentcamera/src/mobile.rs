use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_documentcamera);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<DocumentCamera<R>> {
    #[cfg(target_os = "android")]
    let handle =
        api.register_android_plugin("de.fragdenstaat.scanner.documentcamera", "ExamplePlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_documentcamera)?;
    Ok(DocumentCamera(handle))
}

/// Access to the documentcamera APIs.
pub struct DocumentCamera<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> DocumentCamera<R> {
    pub fn scan(&self, payload: ScanRequest) -> crate::Result<ScanResponse> {
        self.0
            .run_mobile_plugin("scan", payload)
            .map_err(Into::into)
    }
}
