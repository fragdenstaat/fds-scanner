// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod account;
mod api;

use account::{get_user, logout, start_oauth};
use api::{get_foiattachments, get_foimessages, get_foirequests};
use oauth2::CsrfToken;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_store::StoreExt;

#[derive(Clone)]
struct AuthState {
    access_token: String,
    refresh_token: Option<String>,
    expires_at: Option<u64>,
}

impl AuthState {
    fn load(app_handle: &tauri::AppHandle) -> Option<Self> {
        let store = app_handle.store_builder("store.bin").build();

        let access_token = store.get("access_token");
        access_token.map(|token| Self {
            access_token: token.as_str().unwrap().to_string(),
            refresh_token: store
                .get("refresh_token")
                .map(|v| v.as_str().unwrap().to_string()),
            expires_at: store.get("expires_at").map(|v| v.as_u64().unwrap()),
        })
    }

    fn save(&self, app_handle: &tauri::AppHandle) -> Result<(), tauri_plugin_store::Error> {
        let store = app_handle.store_builder("store.bin").build();

        // Note that values must be serde_json::Value instances,
        // otherwise, they will not be compatible with the JavaScript bindings.
        store.set("access_token", self.access_token.clone());

        if self.refresh_token.is_some() {
            store.set("refresh_token", self.refresh_token.clone().unwrap());
        }
        if self.expires_at.is_some() {
            store.set("expires_at", self.expires_at.unwrap());
        }

        store.save()?;
        Ok(())
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct User {
    id: u32,
    first_name: String,
    full_name: String,
    email: String,
}
#[derive(Default)]
struct AppState {
    // login: Option<LoginState>,
    auth: Option<AuthState>,
    user: Option<User>,
}

#[cfg(target_os = "ios")]
fn add_extra_plugins<T>(builder: tauri::Builder<T>) -> tauri::Builder<T>
where
    T: tauri::Runtime,
{
    builder.plugin(tauri_plugin_barcode_scanner::init())
}

#[cfg(not(target_os = "ios"))]
fn add_extra_plugins<T>(builder: tauri::Builder<T>) -> tauri::Builder<T>
where
    T: tauri::Runtime,
{
    builder
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_webauth::init())
        // .plugin(
        //     tauri_plugin_log::Builder::new()
        //         // .cl
        //         // .target(tauri_plugin_log::Target::new(
        //         //     tauri_plugin_log::TargetKind::Stdout,
        //         // ))
        //         .build(),
        // )
        .invoke_handler(tauri::generate_handler![
            get_user,
            start_oauth,
            logout,
            get_foirequests,
            get_foimessages,
            get_foiattachments
        ])
        .setup(|app| {
            app.manage(Mutex::new(AppState {
                auth: AuthState::load(app.handle()),
                user: None,
            }));

            Ok(())
        });

    builder = add_extra_plugins(builder);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
