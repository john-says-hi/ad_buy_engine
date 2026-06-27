use crate::app_state::AppState;
use crate::error::{AppError, AppResult};
use crate::security::hash_token;
use axum::http::header::COOKIE;
use axum::http::HeaderMap;
use chrono::Utc;

pub const SESSION_COOKIE_NAME: &str = "abe_session";

#[derive(Debug, Clone)]
pub struct AuthenticatedAdmin {
    pub username: String,
}

pub async fn authenticate_admin(
    state: &AppState,
    headers: &HeaderMap,
) -> AppResult<AuthenticatedAdmin> {
    let token = read_cookie_value(headers, SESSION_COOKIE_NAME).ok_or(AppError::Unauthorized)?;
    let token_hash = hash_token(&token);
    let now = Utc::now().to_rfc3339();

    let admin = sqlx::query_as::<_, (String,)>(
        r#"
        SELECT admin_users.username
        FROM sessions
        JOIN admin_users ON admin_users.id = sessions.admin_user_id
        WHERE sessions.token_hash = ?1
          AND sessions.expires_at > ?2
        "#,
    )
    .bind(token_hash)
    .bind(now)
    .fetch_optional(&state.pool)
    .await?;

    admin
        .map(|(username,)| AuthenticatedAdmin { username })
        .ok_or(AppError::Unauthorized)
}

pub fn read_cookie_value(headers: &HeaderMap, cookie_name: &str) -> Option<String> {
    let cookie_header = headers.get(COOKIE)?.to_str().ok()?;

    cookie_header.split(';').find_map(|part| {
        let (name, value) = part.trim().split_once('=')?;
        if name == cookie_name {
            Some(value.to_string())
        } else {
            None
        }
    })
}

pub fn build_session_cookie(token: &str, max_age_seconds: i64, secure: bool) -> String {
    let secure_flag = if secure { "; Secure" } else { "" };
    format!(
        "{SESSION_COOKIE_NAME}={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age={max_age_seconds}{secure_flag}"
    )
}

pub fn build_expired_session_cookie(secure: bool) -> String {
    let secure_flag = if secure { "; Secure" } else { "" };
    format!("{SESSION_COOKIE_NAME}=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0{secure_flag}")
}
