use reqwest::header;
use serde::de::DeserializeOwned;

use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};

use crate::account::{AccountError, UserError};
use crate::AppState;

const REQUEST_ENDPOINT: &str = "https://fragdenstaat.de/api/v1/request/";
const MESSAGE_ENDPOINT: &str = "https://fragdenstaat.de/api/v1/message/";
const UPLOAD_ENDPOINT: &str = "https://fragdenstaat.de/api/v1/upload/";
const ATTACHMENT_ENDPOINT: &str = "https://fragdenstaat.de/api/v1/attachment/";

#[derive(Clone, Serialize, Deserialize)]
pub struct FoiRequest {
    id: u32,
    url: String,
    title: String,
    created_at: String,
    last_message: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FoiMessage {
    id: u32,
    timestamp: String,
    is_response: bool,
    sender: String,
    subject: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FoiAttachment {
    id: u32,
    name: String,
    filetype: String,
    size: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    limit: u32,
    next: Option<String>,
    offset: u32,
    previous: Option<String>,
    total_count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    meta: Meta,
    objects: Vec<T>,
}

pub fn get_api_client(
    state: &State<'_, Mutex<AppState>>,
) -> Result<reqwest::Client, reqwest::Error> {
    let access_token = {
        let state = state.lock().unwrap();
        let auth_state = state.auth.as_ref().unwrap();
        auth_state.access_token.clone()
    };
    let mut headers = header::HeaderMap::new();

    headers.insert(
        "Accept",
        header::HeaderValue::from_static("application/json"),
    );
    let mut auth_value =
        header::HeaderValue::from_str(format!("Bearer {}", access_token).as_str()).unwrap();
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);

    reqwest::Client::builder().default_headers(headers).build()
}

fn get_user_id(state: &State<'_, Mutex<AppState>>) -> Result<u32, AccountError> {
    let state = state.lock().unwrap();

    let user_id = match state.user {
        Some(ref user) => user.id,
        None => return Err(UserError("No user found".to_string()).into()),
    };
    Ok(user_id)
}

#[tauri::command]
pub async fn get_foirequests(
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
) -> Result<bool, AccountError> {
    let user_id = get_user_id(&state)?;
    let client = get_api_client(&state)?;

    let mut next = Some(format!("{}?user={}", REQUEST_ENDPOINT, user_id));
    while let Some(next_url) = next {
        let response = client.get(next_url).send().await?;
        let api_response = response.json::<ApiResponse<FoiRequest>>().await?;
        // Emit early to show progress
        app.emit("foirequest-list", &api_response.objects)?;

        next = api_response.meta.next;
    }
    Ok(true)
}

pub async fn get_all_objects<T>(
    url: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<T>, AccountError>
where
    T: Clone + Serialize + DeserializeOwned,
{
    let client = get_api_client(&state)?;

    let mut objects: Vec<T> = vec![];
    let mut next = Some(url);

    while let Some(ref next_url) = next {
        let response = client.get(next_url).send().await?;
        let api_response = response.json::<ApiResponse<T>>().await?;

        objects.extend_from_slice(&api_response.objects);

        next = api_response.meta.next;
    }
    Ok(objects)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_foimessages(
    state: State<'_, Mutex<AppState>>,
    foirequest_id: u32,
) -> Result<Vec<FoiMessage>, AccountError> {
    let url = format!("{}?request={}&kind=post", MESSAGE_ENDPOINT, foirequest_id);
    let objects = get_all_objects(url, state).await?;
    Ok(objects)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_foiattachments(
    state: State<'_, Mutex<AppState>>,
    foimessage_id: u32,
) -> Result<Vec<FoiAttachment>, AccountError> {
    let url = format!("{}?belongs_to={}", ATTACHMENT_ENDPOINT, foimessage_id);
    let objects = get_all_objects(url, state).await?;
    Ok(objects)
}
