use std::path::PathBuf;
use std::sync::Mutex;
use tauri::path::BaseDirectory;
use tauri::{Emitter, Manager, State};
use tauri_plugin_documentcamera::{DocumentCameraExt, ScanRequest};

use crate::api::{create_attachment, create_upload, get_tus_client, resume_upload, FoiAttachment};
use crate::error::AppError;
use crate::AppState;

fn reset_upload_state(
    app_handle: &tauri::AppHandle,
    state: &State<'_, Mutex<AppState>>,
) -> Result<(), AppError> {
    let mut state = state.lock().unwrap();
    state.upload_url = None;
    state.file_path = None;
    state.message_resource_uri = None;
    state.save(app_handle)?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn scan_document(
    app_handle: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
    message_resource_uri: String,
) -> Result<bool, AppError> {
    log::info!("scan document in main called");

    let file_path = app_handle.path().app_local_data_dir()?.join("scan.pdf");
    let result = app_handle.documentcamera().scan(ScanRequest {
        path: file_path.to_str().unwrap().to_string(),
    })?;
    let file_path = match result.path {
        Some(path) => path,
        None => return Ok(false),
    };
    let file_path = PathBuf::from(file_path);

    // // Use this to try without scanning.
    // let file_path = app_handle
    //     .path()
    //     .resolve("resources/example.pdf", BaseDirectory::Resource)?;

    log::info!("scan document completed, file should be at {:?}", file_path);

    if !file_path.exists() {
        return Err(AppError::DocumentCameraResult(format!(
            "File does not exist at {:?}",
            file_path
        )));
    }
    {
        let mut state = state.lock().unwrap();
        state.message_resource_uri = Some(message_resource_uri);
        state.file_path = Some(file_path.to_str().unwrap().to_string());
        state.save(&app_handle)?;
    }
    Ok(true)
}

#[tauri::command]
pub async fn upload_document(
    app: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
) -> Result<Option<FoiAttachment>, AppError> {
    log::info!("upload document document in main called");
    let (message_resource_uri, file_path, upload_url) = {
        let state = state.lock().unwrap();
        (
            state.message_resource_uri.clone(),
            state.file_path.clone(),
            state.upload_url.clone(),
        )
    };

    let (message_resource_uri, file_path) = match (message_resource_uri, file_path) {
        (Some(message_resource_uri), Some(file_path)) => (message_resource_uri, file_path),
        _ => {
            log::warn!("upload_document: missing message_resource_uri or file_path");
            return Ok(None);
        }
    };

    let file_path = PathBuf::from(file_path);
    if !file_path.exists() {
        reset_upload_state(&app, &state)?;
        log::warn!("upload_document: file does not exist at {:?}", file_path);
        return Ok(None);
    }

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

    let result = resume_upload(&tus_client, &upload_url, &file_path).await?;
    if !result {
        reset_upload_state(&app, &state)?;
        log::warn!("upload_document: upload does not exist at {:?}", upload_url);
        return Ok(None);
    }
    app.emit("scan-progress", "upload_complete")?;
    std::fs::remove_file(&file_path)?;

    let att = create_attachment(&state, message_resource_uri, &upload_url).await?;
    app.emit("scan-progress", "attachment_created")?;
    reset_upload_state(&app, &state)?;

    Ok(Some(att))
}
