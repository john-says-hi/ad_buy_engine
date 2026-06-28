use ad_buy_engine_domain::{EntityRecord, EntityRow, SessionResponse};

use crate::route::Route;
use crate::state::entity_form::{EntityKind, FormOptionLists, SaveDraft};
use crate::state::report::ReportDateRange;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use ad_buy_engine_domain::{
        ApiErrorBody, Campaign, CredentialUpdateRequest, Funnel, LandingPage, ListResponse,
        LoginRequest, Offer, OfferSource, OptionsResponse, TrafficSource,
    };
    use gloo_net::http::Request;
    use serde::Serialize;

    use super::*;

    pub async fn get_session() -> Result<SessionResponse, String> {
        get_json("/api/auth/session").await
    }

    pub async fn login(username: String, password: String) -> Result<SessionResponse, String> {
        post_json("/api/auth/login", &LoginRequest { username, password }).await
    }

    pub async fn logout() -> Result<(), String> {
        let response = Request::post("/api/auth/logout")
            .send()
            .await
            .map_err(|error| error.to_string())?;
        if response.ok() {
            Ok(())
        } else {
            Err(error_message(response).await)
        }
    }

    pub async fn update_credentials(
        current_password: String,
        new_username: String,
        new_password: String,
    ) -> Result<SessionResponse, String> {
        put_json(
            "/api/auth/credentials",
            &CredentialUpdateRequest {
                current_password,
                new_username,
                new_password,
            },
        )
        .await
    }

    pub async fn list_rows(
        kind: EntityKind,
        date_range: ReportDateRange,
    ) -> Result<Vec<EntityRow>, String> {
        let response: ListResponse<EntityRow> =
            get_json(&url_with_date_range(kind.endpoint(), date_range)).await?;
        Ok(response.items)
    }

    pub async fn list_report_rows(
        route: Route,
        date_range: ReportDateRange,
    ) -> Result<Vec<EntityRow>, String> {
        let endpoint = route
            .report_rows_endpoint()
            .ok_or_else(|| "This report does not have a data source yet".to_string())?;
        let response: ListResponse<EntityRow> =
            get_json(&url_with_date_range(endpoint, date_range)).await?;
        Ok(response.items)
    }

    pub async fn load_options() -> Result<FormOptionLists, String> {
        Ok(FormOptionLists {
            offer_sources: options("offer-sources").await?,
            offers: options("offers").await?,
            landing_pages: options("landers").await?,
            traffic_sources: options("traffic-sources").await?,
            funnels: options("funnels").await?,
        })
    }

    pub async fn get_entity(kind: EntityKind, id: &str) -> Result<EntityRecord, String> {
        let endpoint = format!("{}/{}", kind.endpoint(), id);
        match kind {
            EntityKind::OfferSource => get_json::<OfferSource>(&endpoint)
                .await
                .map(EntityRecord::OfferSource),
            EntityKind::Offer => get_json::<Offer>(&endpoint).await.map(EntityRecord::Offer),
            EntityKind::LandingPage => get_json::<LandingPage>(&endpoint)
                .await
                .map(EntityRecord::LandingPage),
            EntityKind::TrafficSource => get_json::<TrafficSource>(&endpoint)
                .await
                .map(EntityRecord::TrafficSource),
            EntityKind::Funnel => get_json::<Funnel>(&endpoint)
                .await
                .map(EntityRecord::Funnel),
            EntityKind::Campaign => get_json::<Campaign>(&endpoint)
                .await
                .map(EntityRecord::Campaign),
        }
    }

    pub async fn save_entity(
        kind: EntityKind,
        id: Option<String>,
        draft: SaveDraft,
    ) -> Result<EntityRecord, String> {
        match (kind, id, draft) {
            (EntityKind::OfferSource, Some(id), SaveDraft::OfferSource(draft)) => {
                put_json::<_, OfferSource>(&format!("/api/offer-sources/{id}"), &draft)
                    .await
                    .map(EntityRecord::OfferSource)
            }
            (EntityKind::OfferSource, None, SaveDraft::OfferSource(draft)) => {
                post_json::<_, OfferSource>("/api/offer-sources", &draft)
                    .await
                    .map(EntityRecord::OfferSource)
            }
            (EntityKind::Offer, Some(id), SaveDraft::Offer(draft)) => {
                put_json::<_, Offer>(&format!("/api/offers/{id}"), &draft)
                    .await
                    .map(EntityRecord::Offer)
            }
            (EntityKind::Offer, None, SaveDraft::Offer(draft)) => {
                post_json::<_, Offer>("/api/offers", &draft)
                    .await
                    .map(EntityRecord::Offer)
            }
            (EntityKind::LandingPage, Some(id), SaveDraft::LandingPage(draft)) => {
                put_json::<_, LandingPage>(&format!("/api/landers/{id}"), &draft)
                    .await
                    .map(EntityRecord::LandingPage)
            }
            (EntityKind::LandingPage, None, SaveDraft::LandingPage(draft)) => {
                post_json::<_, LandingPage>("/api/landers", &draft)
                    .await
                    .map(EntityRecord::LandingPage)
            }
            (EntityKind::TrafficSource, Some(id), SaveDraft::TrafficSource(draft)) => {
                put_json::<_, TrafficSource>(&format!("/api/traffic-sources/{id}"), &draft)
                    .await
                    .map(EntityRecord::TrafficSource)
            }
            (EntityKind::TrafficSource, None, SaveDraft::TrafficSource(draft)) => {
                post_json::<_, TrafficSource>("/api/traffic-sources", &draft)
                    .await
                    .map(EntityRecord::TrafficSource)
            }
            (EntityKind::Funnel, Some(id), SaveDraft::Funnel(draft)) => {
                put_json::<_, Funnel>(&format!("/api/funnels/{id}"), &draft)
                    .await
                    .map(EntityRecord::Funnel)
            }
            (EntityKind::Funnel, None, SaveDraft::Funnel(draft)) => {
                post_json::<_, Funnel>("/api/funnels", &draft)
                    .await
                    .map(EntityRecord::Funnel)
            }
            (EntityKind::Campaign, Some(id), SaveDraft::Campaign(draft)) => {
                put_json::<_, Campaign>(&format!("/api/campaigns/{id}"), &draft)
                    .await
                    .map(EntityRecord::Campaign)
            }
            (EntityKind::Campaign, None, SaveDraft::Campaign(draft)) => {
                post_json::<_, Campaign>("/api/campaigns", &draft)
                    .await
                    .map(EntityRecord::Campaign)
            }
            _ => Err("The form data did not match this route".to_string()),
        }
    }

    pub async fn archive_entity(kind: EntityKind, id: String) -> Result<(), String> {
        let response = Request::delete(&format!("{}/{}", kind.endpoint(), id))
            .send()
            .await
            .map_err(|error| error.to_string())?;
        if response.ok() {
            Ok(())
        } else {
            Err(error_message(response).await)
        }
    }

    async fn options(name: &str) -> Result<Vec<ad_buy_engine_domain::OptionItem>, String> {
        let response: OptionsResponse = get_json(&format!("/api/options/{name}")).await?;
        Ok(response.items)
    }

    async fn get_json<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, String> {
        let response = Request::get(url)
            .send()
            .await
            .map_err(|error| error.to_string())?;
        json_response(response).await
    }

    async fn post_json<B: Serialize, T: serde::de::DeserializeOwned>(
        url: &str,
        body: &B,
    ) -> Result<T, String> {
        let response = Request::post(url)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(body).map_err(|error| error.to_string())?)
            .map_err(|error| error.to_string())?
            .send()
            .await
            .map_err(|error| error.to_string())?;
        json_response(response).await
    }

    async fn put_json<B: Serialize, T: serde::de::DeserializeOwned>(
        url: &str,
        body: &B,
    ) -> Result<T, String> {
        let response = Request::put(url)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(body).map_err(|error| error.to_string())?)
            .map_err(|error| error.to_string())?
            .send()
            .await
            .map_err(|error| error.to_string())?;
        json_response(response).await
    }

    async fn json_response<T: serde::de::DeserializeOwned>(
        response: gloo_net::http::Response,
    ) -> Result<T, String> {
        if response.ok() {
            response
                .json::<T>()
                .await
                .map_err(|error| error.to_string())
        } else {
            Err(error_message(response).await)
        }
    }

    async fn error_message(response: gloo_net::http::Response) -> String {
        match response.json::<ApiErrorBody>().await {
            Ok(error) => error.message,
            Err(error) => error.to_string(),
        }
    }

    fn url_with_date_range(endpoint: &str, date_range: ReportDateRange) -> String {
        match date_range_bounds(date_range) {
            Some(bounds) => format!(
                "{endpoint}?start_at_millis={}&end_at_millis={}",
                bounds.start_at_millis, bounds.end_at_millis
            ),
            None => endpoint.to_string(),
        }
    }

    struct DateRangeBounds {
        start_at_millis: i64,
        end_at_millis: i64,
    }

    fn date_range_bounds(date_range: ReportDateRange) -> Option<DateRangeBounds> {
        let today_start = local_midnight_today();
        let tomorrow_start = today_start + MILLIS_PER_DAY;
        match date_range {
            ReportDateRange::Today => Some(DateRangeBounds {
                start_at_millis: today_start,
                end_at_millis: tomorrow_start,
            }),
            ReportDateRange::Yesterday => Some(DateRangeBounds {
                start_at_millis: today_start - MILLIS_PER_DAY,
                end_at_millis: today_start,
            }),
            ReportDateRange::Last3Days => Some(last_days_bounds(today_start, 3)),
            ReportDateRange::Last7Days => Some(last_days_bounds(today_start, 7)),
            ReportDateRange::Last14Days => Some(last_days_bounds(today_start, 14)),
            ReportDateRange::Last30Days => Some(last_days_bounds(today_start, 30)),
            ReportDateRange::Last6Months => Some(last_days_bounds(today_start, 183)),
            ReportDateRange::AllTime => None,
        }
    }

    const MILLIS_PER_DAY: i64 = 86_400_000;

    fn last_days_bounds(today_start: i64, days: i64) -> DateRangeBounds {
        DateRangeBounds {
            start_at_millis: today_start - ((days - 1) * MILLIS_PER_DAY),
            end_at_millis: today_start + MILLIS_PER_DAY,
        }
    }

    fn local_midnight_today() -> i64 {
        let today = js_sys::Date::new_0();
        today.set_hours(0);
        today.set_minutes(0);
        today.set_seconds(0);
        today.set_milliseconds(0);
        today.get_time().round() as i64
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::*;

    pub async fn get_session() -> Result<SessionResponse, String> {
        Err(native_error())
    }

    pub async fn login(_username: String, _password: String) -> Result<SessionResponse, String> {
        Err(native_error())
    }

    pub async fn logout() -> Result<(), String> {
        Err(native_error())
    }

    pub async fn update_credentials(
        _current_password: String,
        _new_username: String,
        _new_password: String,
    ) -> Result<SessionResponse, String> {
        Err(native_error())
    }

    pub async fn list_rows(
        _kind: EntityKind,
        _date_range: ReportDateRange,
    ) -> Result<Vec<EntityRow>, String> {
        Err(native_error())
    }

    pub async fn list_report_rows(
        _route: Route,
        _date_range: ReportDateRange,
    ) -> Result<Vec<EntityRow>, String> {
        Err(native_error())
    }

    pub async fn load_options() -> Result<FormOptionLists, String> {
        Err(native_error())
    }

    pub async fn get_entity(_kind: EntityKind, _id: &str) -> Result<EntityRecord, String> {
        Err(native_error())
    }

    pub async fn save_entity(
        _kind: EntityKind,
        _id: Option<String>,
        _draft: SaveDraft,
    ) -> Result<EntityRecord, String> {
        Err(native_error())
    }

    pub async fn archive_entity(_kind: EntityKind, _id: String) -> Result<(), String> {
        Err(native_error())
    }

    fn native_error() -> String {
        "The dashboard API client only runs in the browser".to_string()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
