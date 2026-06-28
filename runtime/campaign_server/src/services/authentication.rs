use ad_buy_engine_domain::{CredentialUpdateRequest, FieldError, LoginRequest, SessionResponse};
use argon2::{Argon2, PasswordVerifier};
use sqlx::{Row, SqlitePool};
use tower_sessions::Session;

use crate::error::{ServerError, ServerResult};
use crate::storage::database::{hash_password, parse_password_hash};
use crate::time::now_millis;

const SESSION_USERNAME_KEY: &str = "operator_username";

#[derive(Clone, Debug)]
struct OperatorCredentials {
    username: String,
    password_hash: String,
    must_change_credentials: bool,
}

pub async fn login(
    pool: &SqlitePool,
    session: &Session,
    request: LoginRequest,
) -> ServerResult<SessionResponse> {
    let credentials = load_operator_credentials(pool).await?;
    if credentials.username != request.username {
        return Err(ServerError::unauthorized("Invalid username or password"));
    }
    verify_password(&credentials.password_hash, &request.password)?;
    session
        .insert(SESSION_USERNAME_KEY, credentials.username.clone())
        .await
        .map_err(|error| ServerError::internal(format!("failed to write session: {error}")))?;
    Ok(SessionResponse {
        authenticated: true,
        username: Some(credentials.username),
        must_change_credentials: credentials.must_change_credentials,
    })
}

pub async fn logout(session: &Session) -> ServerResult<()> {
    session
        .delete()
        .await
        .map_err(|error| ServerError::internal(format!("failed to clear session: {error}")))?;
    Ok(())
}

pub async fn session_response(
    pool: &SqlitePool,
    session: &Session,
) -> ServerResult<SessionResponse> {
    let Some(username) = session_username(session).await? else {
        return Ok(SessionResponse {
            authenticated: false,
            username: None,
            must_change_credentials: false,
        });
    };
    let credentials = load_operator_credentials(pool).await?;
    if credentials.username != username {
        session.delete().await.map_err(|error| {
            ServerError::internal(format!("failed to clear stale session: {error}"))
        })?;
        return Ok(SessionResponse {
            authenticated: false,
            username: None,
            must_change_credentials: false,
        });
    }

    Ok(SessionResponse {
        authenticated: true,
        username: Some(credentials.username),
        must_change_credentials: credentials.must_change_credentials,
    })
}

pub async fn update_credentials(
    pool: &SqlitePool,
    session: &Session,
    request: CredentialUpdateRequest,
) -> ServerResult<SessionResponse> {
    require_session(pool, session, true).await?;
    validate_credential_update(&request)?;
    let credentials = load_operator_credentials(pool).await?;
    verify_password(&credentials.password_hash, &request.current_password)?;

    let password_hash = hash_password(&request.new_password)?;
    sqlx::query(
        "UPDATE operator_credentials SET
            username = ?, password_hash = ?, must_change_credentials = 0, updated_at_millis = ?
         WHERE id = 1",
    )
    .bind(request.new_username.trim())
    .bind(password_hash)
    .bind(now_millis()?)
    .execute(pool)
    .await?;

    session
        .insert(
            SESSION_USERNAME_KEY,
            request.new_username.trim().to_string(),
        )
        .await
        .map_err(|error| ServerError::internal(format!("failed to refresh session: {error}")))?;

    Ok(SessionResponse {
        authenticated: true,
        username: Some(request.new_username.trim().to_string()),
        must_change_credentials: false,
    })
}

pub async fn require_session(
    pool: &SqlitePool,
    session: &Session,
    allow_must_change: bool,
) -> ServerResult<String> {
    let Some(username) = session_username(session).await? else {
        return Err(ServerError::unauthorized("Login is required"));
    };
    let credentials = load_operator_credentials(pool).await?;
    if credentials.username != username {
        return Err(ServerError::unauthorized("Login is required"));
    }
    if credentials.must_change_credentials && !allow_must_change {
        return Err(ServerError::forbidden(
            "Credentials must be changed before using the dashboard",
        ));
    }
    Ok(username)
}

pub async fn verify_operator_password(pool: &SqlitePool, password: &str) -> ServerResult<()> {
    let credentials = load_operator_credentials(pool).await?;
    verify_password(&credentials.password_hash, password)
}

async fn load_operator_credentials(pool: &SqlitePool) -> ServerResult<OperatorCredentials> {
    let row = sqlx::query(
        "SELECT username, password_hash, must_change_credentials FROM operator_credentials WHERE id = 1",
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ServerError::internal("operator credentials are not seeded"))?;
    Ok(OperatorCredentials {
        username: row.try_get("username")?,
        password_hash: row.try_get("password_hash")?,
        must_change_credentials: row.try_get("must_change_credentials")?,
    })
}

async fn session_username(session: &Session) -> ServerResult<Option<String>> {
    session
        .get(SESSION_USERNAME_KEY)
        .await
        .map_err(|error| ServerError::internal(format!("failed to read session: {error}")))
}

fn verify_password(encoded_hash: &str, candidate: &str) -> ServerResult<()> {
    let parsed_hash = parse_password_hash(encoded_hash)?;
    Argon2::default()
        .verify_password(candidate.as_bytes(), &parsed_hash)
        .map_err(|_| ServerError::unauthorized("Invalid username or password"))
}

fn validate_credential_update(request: &CredentialUpdateRequest) -> ServerResult<()> {
    let mut details = Vec::new();
    if request.current_password.is_empty() {
        details.push(field_error(
            "current_password",
            "Current password is required",
        ));
    }
    if request.new_username.trim().is_empty() {
        details.push(field_error("new_username", "Username is required"));
    }
    if request.new_password.len() < 8 {
        details.push(field_error(
            "new_password",
            "New password must be at least 8 characters",
        ));
    }
    if request.new_username.trim() == "admin" && request.new_password == "admin" {
        details.push(field_error(
            "new_password",
            "Default credentials cannot remain admin/admin",
        ));
    }
    if details.is_empty() {
        Ok(())
    } else {
        Err(ServerError::validation("Credential update failed", details))
    }
}

fn field_error(field: impl Into<String>, message: impl Into<String>) -> FieldError {
    FieldError {
        field: field.into(),
        message: message.into(),
    }
}
