use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{Emitter, Manager, State};
use tauri_plugin_documentcamera::{DocumentCameraExt, ScanRequest};

use crate::api::{create_attachment, create_upload, get_tus_client, resume_upload, MessageId};
use crate::error::AppError;
use crate::AppState;

fn reset_upload_state(
    app_handle: &tauri::AppHandle,
    state: &State<'_, Mutex<AppState>>,
) -> Result<(), AppError> {
    let mut state = state.lock().unwrap();
    state.upload_url = None;
    state.file_path = None;
    state.message_id = None;
    state.save(app_handle)?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn scan_document(
    app_handle: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
    message_id: MessageId,
) -> Result<bool, AppError> {
    log::info!("scan document in main called");

    // Swap this with the commented code below
    // to have a scanned PDF for testing.
    // let file_path = app_handle
    //     .path()
    //     .resolve("resources/example.pdf", BaseDirectory::Resource)
    //     .unwrap();
    let file_path = app_handle.path().app_local_data_dir()?.join("scan.pdf");
    let result = app_handle.documentcamera().scan(ScanRequest {
        path: file_path.to_str().unwrap().to_string(),
    })?;
    if !result.success {
        return Ok(false);
    }
    {
        let mut state = state.lock().unwrap();
        state.message_id = Some(message_id);
        state.file_path = Some(file_path.to_str().unwrap().to_string());
        state.save(&app_handle)?;
    }
    Ok(true)
}

#[tauri::command]
pub async fn upload_document(
    app: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
) -> Result<bool, AppError> {
    log::info!("upload document document in main called");
    let (message_id, file_path, upload_url) = {
        let state = state.lock().unwrap();
        (
            state.message_id,
            state.file_path.clone(),
            state.upload_url.clone(),
        )
    };

    let (message_id, file_path) = match (message_id, file_path) {
        (Some(message_id), Some(file_path)) => (message_id, file_path),
        _ => return Ok(false),
    };

    let file_path = PathBuf::from(file_path);

    let tus_client = get_tus_client(&state)?;

    let upload_url = match upload_url {
        Some(upload_url) => upload_url,
        None => {
            let upload_url = create_upload(&tus_client, &file_path).await?;
            {
                let mut state = state.lock().unwrap();
                state.upload_url = Some(upload_url.clone());
                state.save(&app)?;
            }
            app.emit("scan-progress", "upload_created")?;
            upload_url
        }
    };

    resume_upload(&tus_client, &upload_url, &file_path).await?;
    app.emit("scan-progress", "upload_complete")?;

    create_attachment(&state, message_id, &upload_url).await?;
    app.emit("scan-progress", "attachment_created")?;
    reset_upload_state(&app, &state)?;

    Ok(true)
}
