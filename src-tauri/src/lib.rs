// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod account;
mod api;
mod error;
mod scan;
mod tus;

use account::{get_user, logout, start_oauth};
use api::{get_foiattachments, get_foimessages, get_foirequest, get_foirequests, MessageId};
use scan::{scan_document, upload_document};
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_store::StoreExt;

const SENTRY_DSN: &str = env!("SENTRY_DSN");

#[derive(Clone)]
struct AuthState {
    access_token: String,
    refresh_token: Option<String>,
    expires_at: Option<u64>,
}

type UserId = u32;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct User {
    id: UserId,
    first_name: String,
    full_name: String,
    email: String,
    is_staff: Option<bool>,
}

#[derive(Default)]
struct AppState {
    auth: Option<AuthState>,
    user: Option<User>,
    message_id: Option<MessageId>,
    file_path: Option<String>,
    upload_url: Option<String>,
}

impl AppState {
    fn load(app_handle: &tauri::AppHandle) -> Self {
        let store = app_handle.store_builder("store.bin").build();

        let access_token = store.get("access_token");
        let auth_state = access_token.map(|token| AuthState {
            access_token: token.as_str().unwrap().to_string(),
            refresh_token: store
                .get("refresh_token")
                .map(|v| v.as_str().unwrap().to_string()),
            expires_at: store.get("expires_at").map(|v| v.as_u64().unwrap()),
        });
        AppState {
            auth: auth_state,
            user: None,
            message_id: store.get("message_id").map(|v| v.as_u64().unwrap()),
            file_path: store
                .get("file_path")
                .map(|v| v.as_str().unwrap().to_string()),
            upload_url: store
                .get("upload_url")
                .map(|v| v.as_str().unwrap().to_string()),
        }
    }

    fn save(&self, app_handle: &tauri::AppHandle) -> Result<(), tauri_plugin_store::Error> {
        let store = app_handle.store_builder("store.bin").build();

        // Note that values must be serde_json::Value instances,
        // otherwise, they will not be compatible with the JavaScript bindings.
        match self.auth {
            Some(ref auth) => {
                store.set("access_token", auth.access_token.clone());
                if let Some(ref refresh_token) = auth.refresh_token {
                    store.set("refresh_token", refresh_token.clone());
                }
                if let Some(expires_at) = auth.expires_at {
                    store.set("expires_at", expires_at);
                }
            }
            None => {
                store.delete("access_token");
                store.delete("refresh_token");
                store.delete("expires_at");
            }
        }
        if let Some(message_id) = self.message_id {
            store.set("message_id", message_id);
        } else {
            store.delete("message_id");
        }

        if let Some(ref file_path) = self.file_path {
            store.set("file_path", file_path.clone());
        } else {
            store.delete("file_path");
        }
        if let Some(ref upload_url) = self.upload_url {
            store.set("upload_url", upload_url.clone());
        } else {
            store.delete("upload_url");
        }

        store.save()?;
        Ok(())
    }
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
    let _guard = sentry::init((
        SENTRY_DSN,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_webauth::init())
        .plugin(tauri_plugin_documentcamera::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                ))
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            get_user,
            start_oauth,
            logout,
            get_foirequests,
            get_foirequest,
            get_foimessages,
            get_foiattachments,
            scan_document,
            upload_document,
        ])
        .setup(|app| {
            app.manage(Mutex::new(AppState::load(app.handle())));
            Ok(())
        });

    builder = add_extra_plugins(builder);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
