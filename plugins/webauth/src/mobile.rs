use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_webauth);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<WebAuth<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("de.fragdenstaat.scanner.webauth", "WebAuthPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_webauth)?;
    Ok(WebAuth(handle))
}

/// Access to the webauth APIs.
pub struct WebAuth<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> WebAuth<R> {
    pub fn start_auth(&self, payload: WebAuthRequest) -> crate::Result<WebAuthResponse> {
        println!("start auth in plugin called: {}", payload.url);
        self.0
            .run_mobile_plugin("start_auth", payload)
            .map_err(Into::into)
    }
}
