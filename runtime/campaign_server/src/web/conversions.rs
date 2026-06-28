use std::collections::HashMap;

use ad_buy_engine_domain::ConversionTrackingResponse;
use axum::Json;
use axum::body::{Bytes, to_bytes};
use axum::extract::{Query, Request, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderMap, HeaderValue};
use axum::response::{IntoResponse, Response};

use crate::error::{ServerError, ServerResult};
use crate::storage::conversions::{IncomingConversion, track_conversion};
use crate::web::router::AppState;

const BODY_LIMIT_BYTES: usize = 64 * 1024;
const TRANSPARENT_GIF: &[u8] = &[
    71, 73, 70, 56, 57, 97, 1, 0, 1, 0, 128, 0, 0, 0, 0, 0, 255, 255, 255, 33, 249, 4, 1, 0, 0, 0,
    0, 44, 0, 0, 0, 0, 1, 0, 1, 0, 0, 2, 2, 68, 1, 0, 59,
];

pub async fn postback(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
    request: Request,
) -> ServerResult<Json<ConversionTrackingResponse>> {
    let params = request_params(query, request).await?;
    track_conversion(
        &state.pool,
        IncomingConversion {
            source: "postback",
            params,
        },
    )
    .await
    .map(Json)
}

pub async fn conversion_gif(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
    request: Request,
) -> ServerResult<Response> {
    let params = request_params(query, request).await?;
    track_conversion(
        &state.pool,
        IncomingConversion {
            source: "pixel",
            params,
        },
    )
    .await?;
    Ok((
        [(CONTENT_TYPE, HeaderValue::from_static("image/gif"))],
        TRANSPARENT_GIF,
    )
        .into_response())
}

async fn request_params(
    query: HashMap<String, String>,
    request: Request,
) -> ServerResult<Vec<(String, String)>> {
    let (parts, body) = request.into_parts();
    let body = to_bytes(body, BODY_LIMIT_BYTES)
        .await
        .map_err(|error| ServerError::bad_request(format!("Invalid postback body: {error}")))?;
    let mut params = sorted_pairs(query);
    params.extend(body_params(&parts.headers, body)?);
    Ok(params)
}

fn body_params(headers: &HeaderMap, body: Bytes) -> ServerResult<Vec<(String, String)>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let content_type = headers
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    if content_type.starts_with("application/json") {
        return json_body_params(&body);
    }
    if content_type.starts_with("application/x-www-form-urlencoded") {
        let mut pairs = url::form_urlencoded::parse(&body)
            .map(|(key, value)| (key.into_owned(), value.into_owned()))
            .collect::<Vec<_>>();
        pairs.sort_by(|left, right| left.0.cmp(&right.0));
        return Ok(pairs);
    }
    Ok(Vec::new())
}

fn json_body_params(body: &[u8]) -> ServerResult<Vec<(String, String)>> {
    let value: serde_json::Value = serde_json::from_slice(body)
        .map_err(|error| ServerError::bad_request(format!("Invalid JSON postback: {error}")))?;
    let Some(object) = value.as_object() else {
        return Err(ServerError::bad_request(
            "JSON postback body must be an object",
        ));
    };
    let mut pairs = object
        .iter()
        .map(|(key, value)| (key.clone(), json_value_to_string(value)))
        .collect::<Vec<_>>();
    pairs.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(pairs)
}

fn json_value_to_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => String::new(),
        serde_json::Value::String(value) => value.clone(),
        serde_json::Value::Bool(value) => value.to_string(),
        serde_json::Value::Number(value) => value.to_string(),
        _ => value.to_string(),
    }
}

fn sorted_pairs(query: HashMap<String, String>) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = query.into_iter().collect();
    pairs.sort_by(|left, right| left.0.cmp(&right.0));
    pairs
}
