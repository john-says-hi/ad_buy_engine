use axum::body::to_bytes;
use axum::http::header::{CONTENT_TYPE, LOCATION};
use axum::http::{Request, StatusCode};
use clap::Parser;
use fake_landing_page_server::build_router;
use fake_landing_page_server::cli::Cli;
use fake_landing_page_server::config::RunConfig;
use tower::ServiceExt;

fn config_from(args: &[&str]) -> Result<RunConfig, Box<dyn std::error::Error>> {
    let cli = Cli::try_parse_from(
        std::iter::once("abe-fake-landing-page-server").chain(args.iter().copied()),
    )?;
    Ok(RunConfig::try_from(cli)?)
}

async fn post_form(
    uri: &str,
    body: &str,
    config: RunConfig,
) -> Result<(StatusCode, Option<String>, String), Box<dyn std::error::Error>> {
    let response = build_router(config)
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(axum::body::Body::from(body.to_string()))?,
        )
        .await?;
    let status = response.status();
    let location = response
        .headers()
        .get(LOCATION)
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned);
    let bytes = to_bytes(response.into_body(), usize::MAX).await?;
    Ok((status, location, String::from_utf8(bytes.to_vec())?))
}

#[tokio::test]
async fn lead_capture_submit_redirects_to_supplied_continuation_without_email()
-> Result<(), Box<dyn std::error::Error>> {
    let target = "http%3A%2F%2F127.0.0.1%3A8088%2Fgo%2Fvisit-lead%2F1";
    let body = format!("email=person%40example.test&first_name=Fake&next={target}");
    let (status, location, response_body) = post_form(
        "/lander/fake-lander-lead-capture/opt-in",
        &body,
        config_from(&[])?,
    )
    .await?;

    assert_eq!(status, StatusCode::SEE_OTHER);
    assert_eq!(
        location.as_deref(),
        Some("http://127.0.0.1:8088/go/visit-lead/1")
    );
    assert!(
        !location
            .unwrap_or_default()
            .contains("person%40example.test")
    );
    assert!(!response_body.contains("person@example.test"));
    Ok(())
}

#[tokio::test]
async fn lead_capture_submit_rejects_unsafe_targets_and_discards_input()
-> Result<(), Box<dyn std::error::Error>> {
    let (status, location, response_body) = post_form(
        "/lander/fake-lander-lead-capture/opt-in",
        "email=person%40example.test&next=https%3A%2F%2Fpublic.example%2Fgo",
        config_from(&[])?,
    )
    .await?;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(location, None);
    assert!(response_body.contains("Blocked continuation target in next"));
    assert!(!response_body.contains("person@example.test"));
    assert!(!response_body.contains("/postback"));
    assert!(!response_body.contains("/conversion.gif"));
    Ok(())
}

#[tokio::test]
async fn lead_capture_submit_missing_target_returns_controlled_error()
-> Result<(), Box<dyn std::error::Error>> {
    let (status, location, response_body) = post_form(
        "/lander/fake-lander-lead-capture/opt-in",
        "email=person%40example.test",
        config_from(&[])?,
    )
    .await?;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(location, None);
    assert!(response_body.contains("Missing continuation target: next"));
    assert!(!response_body.contains("person@example.test"));
    Ok(())
}

#[tokio::test]
async fn repeated_fake_email_values_do_not_change_routing() -> Result<(), Box<dyn std::error::Error>>
{
    let first = post_form(
        "/lander/fake-lander-lead-capture/opt-in",
        "email=first%40example.test&next=http%3A%2F%2F127.0.0.1%3A8088%2Fgo%2Fvisit-repeat%2F1",
        config_from(&[])?,
    )
    .await?;
    let second = post_form(
        "/lander/fake-lander-lead-capture/opt-in",
        "email=second%40example.test&next=http%3A%2F%2F127.0.0.1%3A8088%2Fgo%2Fvisit-repeat%2F1",
        config_from(&[])?,
    )
    .await?;

    assert_eq!(first.0, StatusCode::SEE_OTHER);
    assert_eq!(second.0, StatusCode::SEE_OTHER);
    assert_eq!(first.1, second.1);
    Ok(())
}

#[tokio::test]
async fn non_lead_lander_cannot_use_opt_in_submit_route() -> Result<(), Box<dyn std::error::Error>>
{
    let (status, location, response_body) = post_form(
        "/lander/fake-lander-standard-click-through/opt-in",
        "email=person%40example.test&next=http%3A%2F%2F127.0.0.1%3A8088%2Fgo%2Fvisit%2F1",
        config_from(&[])?,
    )
    .await?;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(location, None);
    assert!(response_body.contains("only available for the fake lead-capture lander"));
    Ok(())
}
