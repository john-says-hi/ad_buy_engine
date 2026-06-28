use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::response::Redirect;
use serde::Deserialize;

use crate::error::{ServerError, ServerResult};
use crate::services::click_processor::{process_campaign_click, process_lander_click};
use crate::web::router::AppState;

#[derive(Clone, Debug, Deserialize)]
pub struct RawQuery(std::collections::HashMap<String, String>);

pub async fn campaign_click(
    State(state): State<AppState>,
    Path(campaign_id): Path<String>,
    headers: HeaderMap,
    Query(query): Query<RawQuery>,
) -> ServerResult<Redirect> {
    let raw_query = serde_urlencoded(query.0)?;
    let outcome = process_campaign_click(
        &state.pool,
        &state.public_base_url,
        &campaign_id,
        &headers,
        Some(&raw_query),
    )
    .await?;
    redirect(&outcome.target)
}

pub async fn lander_click(
    State(state): State<AppState>,
    Path((visit_id, slot)): Path<(String, u8)>,
) -> ServerResult<Redirect> {
    let outcome = process_lander_click(&state.pool, &visit_id, slot).await?;
    redirect(&outcome.target)
}

fn redirect(target: &str) -> ServerResult<Redirect> {
    if target.trim().is_empty() {
        Err(ServerError::bad_request("Redirect target is empty"))
    } else {
        Ok(Redirect::temporary(target))
    }
}

fn serde_urlencoded(query: std::collections::HashMap<String, String>) -> ServerResult<String> {
    let mut pairs: Vec<(String, String)> = query.into_iter().collect();
    pairs.sort_by(|left, right| left.0.cmp(&right.0));
    let encoded = pairs
        .into_iter()
        .map(|(key, value)| {
            format!(
                "{}={}",
                urlencoding::encode(&key),
                urlencoding::encode(&value)
            )
        })
        .collect::<Vec<_>>()
        .join("&");
    Ok(encoded)
}
