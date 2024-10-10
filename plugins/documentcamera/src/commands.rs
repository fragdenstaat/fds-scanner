use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::DocumentCameraExt;
use crate::Result;

#[command]
pub(crate) async fn scan<R: Runtime>(
    app: AppHandle<R>,
    payload: ScanRequest,
) -> Result<ScanResponse> {
    app.documentcamera().scan(payload)
}
