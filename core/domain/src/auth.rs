use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialUpdateRequest {
    pub current_password: String,
    pub new_username: String,
    pub new_password: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionResponse {
    pub authenticated: bool,
    pub username: Option<String>,
    pub must_change_credentials: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthResponse {
    pub ok: bool,
    pub app_version: String,
    pub schema_version: i64,
}
