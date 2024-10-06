use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::WebAuth;
#[cfg(mobile)]
use mobile::WebAuth;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the webauth APIs.
pub trait WebAuthExt<R: Runtime> {
    fn webauth(&self) -> &WebAuth<R>;
}

impl<R: Runtime, T: Manager<R>> crate::WebAuthExt<R> for T {
    fn webauth(&self) -> &WebAuth<R> {
        self.state::<WebAuth<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("webauth")
        .invoke_handler(tauri::generate_handler![commands::start_auth])
        .setup(|app, api| {
            #[cfg(mobile)]
            let webauth = mobile::init(app, api)?;
            #[cfg(desktop)]
            let webauth = desktop::init(app, api)?;
            app.manage(webauth);
            Ok(())
        })
        .build()
}
