use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::Result;
use crate::WebAuthExt;

#[command]
pub(crate) async fn start_auth<R: Runtime>(
    app: AppHandle<R>,
    payload: WebAuthRequest,
) -> Result<WebAuthResponse> {
    app.webauth().start_auth(payload)
}
