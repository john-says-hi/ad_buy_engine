use crate::utils::authentication::{decode_jwt, PrivateClaim};
use crate::utils::errors::ApiError;
use crate::utils::helpers::respond_json;
use actix_identity::Identity;
use actix_web::web::{block, Data, Json};
use actix_web::HttpResponse;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

pub async fn get_team_id(id: Identity) -> Result<Json<Uuid>, ApiError> {
    let restored_identity: PrivateClaim =
        decode_jwt(&id.identity().expect("g3qw")).map_err(|e| e)?;

    respond_json(restored_identity.account_id)
}

pub async fn get_health() -> Result<Json<HealthResponse>, ApiError> {
    respond_json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
    })
}

pub async fn check_is_logged_in() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_get_health() {
        let response = get_health().await.unwrap();
        assert_eq!(response.into_inner().status, "ok".to_string());
    }
}
