use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::reqwest::async_http_client;
use oauth2::url::Url;
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, CsrfToken, EmptyExtraTokenFields,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RefreshToken, RevocationUrl, Scope,
    StandardRevocableToken, StandardTokenResponse, TokenResponse, TokenUrl,
};
use std::sync::Mutex;
use tauri::State;
use tauri_plugin_webauth::{WebAuthExt, WebAuthRequest};

use crate::api::get_api_client;
use crate::error::{AppError, AuthorizationError, UserError};
use crate::{AppState, AuthState, User};

const REDIRECT_URI: &str = "fragdenstaat://loggedin";
const CLIENT_ID: &str = "1nmNtPIiQ7xA1yqzZDwEmOlguNEhdnp5vQpGyfSd";
const HOSTNAME: &str = "fragdenstaat.de";
const AUTHORIZE_ENDPOINT: &str = "https://fragdenstaat.de/account/authorize/";
const ACCESS_TOKEN_ENDPOINT: &str = "https://fragdenstaat.de/account/token/";
const REVOKE_TOKEN_ENDPOINT: &str = "https://fragdenstaat.de/account/revoke_token/";

const USER_ENDPOINT: &str = "https://fragdenstaat.de/api/v1/user/";

const SCOPE: [&str; 5] = [
    "read:user",
    "read:profile",
    "read:email",
    "read:request",
    "upload:message",
];

struct OAuthData {
    auth_url: Url,
    pkce_verifier: PkceCodeVerifier,
    csrf_token: CsrfToken,
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
pub async fn get_user(
    app_handle: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
) -> Result<User, AppError> {
    log::info!("get user in main called");

    {
        let state = state.lock().unwrap();
        if let Some(user) = &state.user {
            log::debug!("Found user in state: {:?}", user);
            return Ok(user.clone());
        }
    }

    {
        let state = state.lock().unwrap();
        if state.auth.is_none() {
            return Err(UserError("No auth state found".to_string()).into());
        }
    };

    let mut tries = 0;
    let mut response;

    loop {
        tries += 1;
        let client = get_api_client(&state)?;
        let request = client.get(USER_ENDPOINT);
        log::debug!("Requesting user data... {:?}", request);

        response = request.send().await?;
        if response.status().is_client_error() && tries < 2 {
            refresh_token(&app_handle, &state).await?;
            continue;
        }
        break;
    }

    let user = response.json::<User>().await?;
    log::debug!("Received user: {:?}", user);

    {
        let mut state = state.lock().unwrap();
        state.user = Some(user.clone());
    }

    Ok(user)
}

pub async fn refresh_token(
    app_handle: &tauri::AppHandle,
    state: &State<'_, Mutex<AppState>>,
) -> Result<bool, AppError> {
    let oauth2_client = get_outh2_client()?;
    let refresh_token = {
        let state = state.lock().unwrap();
        match state.auth {
            Some(ref auth_state) => auth_state
                .refresh_token
                .clone()
                .ok_or(AuthorizationError("Missing refresh token".to_string())),
            None => Err(AuthorizationError("Missing auth state".to_string())),
        }
    }?;

    let token_result = oauth2_client
        .exchange_refresh_token(&RefreshToken::new(refresh_token))
        .request_async(async_http_client)
        .await;
    let token_result = match token_result {
        Ok(token) => Ok(token),
        Err(_err) => Err(AuthorizationError(
            "Could not refresh access token".to_string(),
        )),
    }?;

    store_token_result(app_handle, state, token_result)?;

    Ok(true)
}

#[tauri::command]
pub async fn start_oauth(
    app_handle: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
    start_url: Option<String>,
) -> Result<bool, AppError> {
    log::info!("start auth in main called");

    let verified_start_url = match start_url {
        Some(url) => {
            let url = Url::parse(url.as_str())?;
            if url.host_str() != Some(HOSTNAME) {
                return Err(AuthorizationError("Invalid start URL".to_string()).into());
            }
            Some(url)
        }
        None => None,
    };

    let oauth2_client = get_outh2_client()?;
    let auth_data = get_oauth_url(&oauth2_client)?;

    let auth_url = match verified_start_url {
        Some(url) => {
            let mut auth_url = url;
            auth_url
                .query_pairs_mut()
                .append_pair("next", auth_data.auth_url.as_str());
            auth_url
        }
        None => auth_data.auth_url,
    };

    let auth_response = app_handle.webauth().start_auth(WebAuthRequest {
        url: auth_url.to_string(),
        redirect_url: REDIRECT_URI.to_string(),
    })?;

    let return_url = match auth_response.url {
        Some(url) => url,
        None => return Err(AuthorizationError("Invalid return URL".to_string()).into()),
    };
    log::info!("Received auth response: {}", return_url);
    if !return_url.starts_with(REDIRECT_URI) {
        return Err(AuthorizationError(format!("Mismatching return URL: {}", return_url)).into());
    }
    let return_url = Url::parse(return_url.as_str())?;
    let query_params = return_url.query_pairs();

    let mut authorization_code: Option<String> = None;
    let mut state_param: Option<String> = None;

    for (key, value) in query_params {
        if key == "error" {
            return Err(AuthorizationError(format!("error in query params: {}", value)).into());
        }
        if key == "code" {
            authorization_code = Some(value.to_string());
        }
        if key == "state" {
            state_param = Some(value.to_string());
        }
    }

    let authorization_code = match authorization_code {
        Some(code) => code,
        None => return Err(AuthorizationError("No code found in response".to_string()).into()),
    };
    let authorization_code = AuthorizationCode::new(authorization_code);

    let state_param = match state_param {
        Some(state) => state,
        None => return Err(AuthorizationError("No state found in response".to_string()).into()),
    };

    if *auth_data.csrf_token.secret() != state_param {
        return Err(AuthorizationError("State does not match".to_string()).into());
    }

    let pkce_verifier = auth_data.pkce_verifier;
    let token_result = oauth2_client
        .exchange_code(authorization_code)
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await;
    let token_result = match token_result {
        Ok(token) => token,
        Err(err) => match err {
            oauth2::RequestTokenError::ServerResponse(response) => {
                log::error!(
                    "{}",
                    response
                        .error_description()
                        .unwrap_or(&"No error description".to_string())
                );
                log::error!("Failed to request token: {:?}", response);
                return Err(AuthorizationError("Server response error".to_string()).into());
            }
            oauth2::RequestTokenError::Request(err) => {
                log::error!("Failed to request token: {:?}", err);
                return Err(AuthorizationError("Request error".to_string()).into());
            }
            _ => {
                log::error!("Failed to request token: {:?}", err);
                return Err(AuthorizationError("Parse or other error".to_string()).into());
            }
        },
    };

    // let expires_at = match token_result.expires_in() {
    //     Some(expires_in) => {
    //         Some(SystemTime::now().
    //             .duration_since(SystemTime::UNIX_EPOCH)
    //             .unwrap() + expires_in)
    //     }
    //     None => None
    // };
    //

    store_token_result(&app_handle, &state, token_result)?;

    Ok(true)
}

fn store_token_result(
    app_handle: &tauri::AppHandle,
    state: &State<'_, Mutex<AppState>>,
    token_result: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
) -> Result<(), AppError> {
    let auth_state = AuthState {
        access_token: token_result.access_token().secret().to_string(),
        refresh_token: token_result.refresh_token().map(|x| x.secret().to_string()),
        expires_at: None,
    };
    log::info!("Received access token: {}", auth_state.access_token);
    {
        let mut state = state.lock().unwrap();
        state.auth = Some(auth_state);
        state.save(app_handle)?;
    }
    Ok(())
}

fn get_outh2_client() -> Result<BasicClient, AppError> {
    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = BasicClient::new(
        ClientId::new(CLIENT_ID.to_string()),
        None,
        AuthUrl::new(AUTHORIZE_ENDPOINT.to_string())?,
        Some(TokenUrl::new(ACCESS_TOKEN_ENDPOINT.to_string())?),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new(REDIRECT_URI.to_string())?)
    .set_revocation_uri(RevocationUrl::new(REVOKE_TOKEN_ENDPOINT.to_string())?);
    Ok(client)
}

fn get_oauth_url(client: &BasicClient) -> Result<OAuthData, AppError> {
    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let mut auth_request = client.authorize_url(CsrfToken::new_random);
    // Set the desired scopes.

    for scope in SCOPE.iter() {
        auth_request = auth_request.add_scope(Scope::new(scope.to_string()));
    }
    // Set the PKCE code challenge.
    auth_request = auth_request.set_pkce_challenge(pkce_challenge);
    // Generate the full authorization URL.
    let (auth_url, csrf_token) = auth_request.url();

    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    Ok(OAuthData {
        auth_url,
        pkce_verifier,
        csrf_token,
    })
}

#[tauri::command]
pub async fn logout(
    app_handle: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
) -> Result<bool, AppError> {
    log::info!("start logout in main called");

    let oauth2_client = get_outh2_client()?;

    let (access_token, refresh_token) = {
        let state = state.lock().unwrap();
        match &state.auth {
            Some(auth_state) => (
                auth_state.access_token.clone(),
                auth_state.refresh_token.clone(),
            ),
            None => {
                return Err(UserError("No auth state found".to_string()).into());
            }
        }
    };
    oauth2_client.revoke_token(StandardRevocableToken::AccessToken(AccessToken::new(
        access_token,
    )))?;
    if let Some(refresh_token) = refresh_token {
        oauth2_client.revoke_token(StandardRevocableToken::RefreshToken(RefreshToken::new(
            refresh_token,
        )))?;
    }

    {
        let mut state = state.lock().unwrap();
        state.auth = None;
        state.user = None;
        state.upload_url = None;
        state.file_path = None;
        state.message_id = None;
        state.save(&app_handle)?;
    }
    Ok(true)
}
