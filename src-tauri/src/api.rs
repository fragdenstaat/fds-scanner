use reqwest::header;
use serde::de::DeserializeOwned;

use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};

use crate::error::{AppError, TusError, UserError};
use crate::tus::TusClient;
use crate::{AppState, UserId};
use chrono::prelude::*;

const REQUEST_ENDPOINT: &str = "https://fragdenstaat.de/api/v1/request/";
const MESSAGE_ENDPOINT: &str = "https://fragdenstaat.de/api/v1/message/";
const UPLOAD_ENDPOINT: &str = "https://fragdenstaat.de/api/v1/upload/";
const ATTACHMENT_ENDPOINT: &str = "https://fragdenstaat.de/api/v1/attachment/";
const UPLOAD_URL_BASE: &str = "https://fragdenstaat.de";

type FoiRequestId = u64;

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicBody {
    id: u64,
    name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FoiRequest {
    id: FoiRequestId,
    url: String,
    title: String,
    created_at: String,
    last_message: String,
    public_body: PublicBody,
}

pub type MessageId = u64;

#[derive(Clone, Serialize, Deserialize)]
pub struct FoiMessage {
    id: MessageId,
    timestamp: String,
    is_response: bool,
    sender: String,
    subject: String,
}

pub type AttachmentId = u64;

#[derive(Clone, Serialize, Deserialize)]
pub struct FoiAttachment {
    id: AttachmentId,
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

fn get_user_id(state: &State<'_, Mutex<AppState>>) -> Result<UserId, AppError> {
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
) -> Result<bool, AppError> {
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
) -> Result<Vec<T>, AppError>
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
    foirequest_id: FoiRequestId,
) -> Result<Vec<FoiMessage>, AppError> {
    let url = format!("{}?request={}&kind=post", MESSAGE_ENDPOINT, foirequest_id);
    let objects = get_all_objects(url, state).await?;
    Ok(objects)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_foiattachments(
    state: State<'_, Mutex<AppState>>,
    foimessage_id: u32,
) -> Result<Vec<FoiAttachment>, AppError> {
    let url = format!("{}?belongs_to={}", ATTACHMENT_ENDPOINT, foimessage_id);
    let objects = get_all_objects(url, state).await?;
    Ok(objects)
}

pub fn get_tus_client(state: &State<'_, Mutex<AppState>>) -> Result<TusClient, AppError> {
    let req_client = get_api_client(state)?;
    Ok(TusClient::new(req_client))
}

pub async fn create_upload(client: &TusClient, file_path: &Path) -> Result<String, AppError> {
    let local: DateTime<Local> = Local::now();
    let current_date = local.format("%d-%m-%Y").to_string();

    let mut metadata = HashMap::new();
    metadata.insert("filetype".to_string(), "application/pdf".to_string());
    metadata.insert("filename".to_string(), format!("scan_{}.pdf", current_date));

    let upload_url = client
        .create_with_metadata(UPLOAD_ENDPOINT, file_path, metadata)
        .await?;

    log::info!("Upload URL: {}", upload_url);
    // let upload_url = Url::parse(&upload_url).unwrap();
    // let upload_url = match upload_url.host_str() {
    //     Some(_) => upload_url.to_string(),
    //     _ => {
    //         format!("{}{}", UPLOAD_URL_BASE, upload_url)
    //     }
    // };
    let upload_url = format!("{}{}", UPLOAD_URL_BASE, upload_url);

    Ok(upload_url)
}

pub async fn resume_upload(
    client: &TusClient,
    upload_url: &str,
    file_path: &Path,
) -> Result<bool, AppError> {
    let response = client.upload(upload_url, file_path).await;
    match response {
        Ok(_) => Ok(true),
        Err(e) => match e {
            // Reset the upload state if the upload URL is not found
            TusError::NotFoundError => Ok(false),
            _ => Err(e.into()),
        },
    }
}

#[derive(Serialize)]
struct CreateAttachment {
    message: MessageId,
    upload: String,
}

pub async fn create_attachment(
    state: &State<'_, Mutex<AppState>>,
    message_id: MessageId,
    upload_url: &str,
) -> Result<FoiAttachment, AppError> {
    let client = get_api_client(state)?;

    let att_data = CreateAttachment {
        message: message_id,
        upload: upload_url.to_string(),
    };

    let response = client
        .post(ATTACHMENT_ENDPOINT)
        .json(&att_data)
        .send()
        .await?;
    response.status().is_success();
    let attachment = response.json::<FoiAttachment>().await?;
    Ok(attachment)
}
