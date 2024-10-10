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
use desktop::DocumentCamera;
#[cfg(mobile)]
use mobile::DocumentCamera;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the documentcamera APIs.
pub trait DocumentCameraExt<R: Runtime> {
    fn documentcamera(&self) -> &DocumentCamera<R>;
}

impl<R: Runtime, T: Manager<R>> crate::DocumentCameraExt<R> for T {
    fn documentcamera(&self) -> &DocumentCamera<R> {
        self.state::<DocumentCamera<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("documentcamera")
        .invoke_handler(tauri::generate_handler![commands::scan])
        .setup(|app, api| {
            #[cfg(mobile)]
            let documentcamera = mobile::init(app, api)?;
            #[cfg(desktop)]
            let documentcamera = desktop::init(app, api)?;
            app.manage(documentcamera);
            Ok(())
        })
        .build()
}
