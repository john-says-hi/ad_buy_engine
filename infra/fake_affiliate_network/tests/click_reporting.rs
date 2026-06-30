use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use clap::Parser;
use fake_affiliate_network::build_router;
use fake_affiliate_network::cli::Cli;
use fake_affiliate_network::config::RunConfig;
use tower::ServiceExt;

fn config_from(args: &[&str]) -> Result<RunConfig, Box<dyn std::error::Error>> {
    let cli = Cli::try_parse_from(
        std::iter::once("abe-fake-affiliate-network").chain(args.iter().copied()),
    )?;
    Ok(RunConfig::try_from(cli)?)
}

async fn response_text(
    response: axum::response::Response,
) -> Result<String, Box<dyn std::error::Error>> {
    let body = to_bytes(response.into_body(), 128 * 1024).await?;
    Ok(String::from_utf8(body.to_vec())?)
}

#[tokio::test]
async fn dashboard_lists_all_fake_offers_with_tracking_links()
-> Result<(), Box<dyn std::error::Error>> {
    let app = build_router(config_from(&[])?)?;

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty())?)
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    let body = response_text(response).await?;
    assert!(body.contains("Fake Solar Savings Lead"));
    assert!(body.contains("Fake Fitness Trial Lead"));
    assert!(body.contains("Fake Credit Checkup Lead"));
    assert!(body.contains("Fake Course Bundle Sale"));
    assert!(body.contains("Fake Smart Garden Kit Sale"));
    assert!(body.contains("/click/fake-lead-solar-savings?subid={clickid}"));
    Ok(())
}

#[tokio::test]
async fn click_route_records_subid_aliases_and_unattributed_clicks()
-> Result<(), Box<dyn std::error::Error>> {
    let app = build_router(config_from(&[])?)?;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/click/fake-lead-solar-savings?subid=visit-123&utm_source=local")
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/click/fake-lead-solar-savings?click_id=visit-123")
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/click/fake-lead-solar-savings")
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let dashboard = app
        .oneshot(Request::builder().uri("/").body(Body::empty())?)
        .await?;
    let body = response_text(dashboard).await?;
    assert!(body.contains("<td>visit-123</td><td>2</td>"));
    assert!(body.contains("<td>unattributed</td><td>1</td>"));
    assert!(!body.contains("local@example.com"));
    Ok(())
}

#[tokio::test]
async fn unknown_offer_click_returns_not_found_without_recording()
-> Result<(), Box<dyn std::error::Error>> {
    let app = build_router(config_from(&[])?)?;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/click/unknown-offer?subid=visit-123")
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let dashboard = app
        .oneshot(Request::builder().uri("/").body(Body::empty())?)
        .await?;
    let body = response_text(dashboard).await?;
    assert!(body.contains("No clicks recorded in this run."));
    Ok(())
}
