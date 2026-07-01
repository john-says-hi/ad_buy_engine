use axum::body::to_bytes;
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

async fn get_body(
    uri: &str,
    config: RunConfig,
) -> Result<(StatusCode, String), Box<dyn std::error::Error>> {
    let response = build_router(config)
        .oneshot(
            Request::builder()
                .uri(uri)
                .body(axum::body::Body::empty())?,
        )
        .await?;
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await?;
    Ok((status, String::from_utf8(bytes.to_vec())?))
}

#[tokio::test]
async fn standard_click_through_renders_primary_cta() -> Result<(), Box<dyn std::error::Error>> {
    let (status, body) = get_body(
        "/lander/fake-lander-standard-click-through?next=http%3A%2F%2F127.0.0.1%3A8088%2Fgo%2Fvisit-1%2F1",
        config_from(&[])?,
    )
    .await?;

    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Fake Standard Click-Through Lander"));
    assert!(body.contains("Fake landing page preset: fake-lander-standard-click-through"));
    assert!(body.contains("href=\"http://127.0.0.1:8088/go/visit-1/1\""));
    assert!(body.contains("data-continuation=\"next\""));
    Ok(())
}

#[tokio::test]
async fn advertorial_page_renders_article_style_cta() -> Result<(), Box<dyn std::error::Error>> {
    let (status, body) = get_body(
        "/lander/fake-lander-advertorial?next=http%3A%2F%2F127.0.0.1%3A8088%2Fgo%2Fvisit-2%2F1",
        config_from(&[])?,
    )
    .await?;

    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Fake Advertorial Lander"));
    assert!(body.contains("Fake field report"));
    assert!(body.contains("href=\"http://127.0.0.1:8088/go/visit-2/1\""));
    Ok(())
}

#[tokio::test]
async fn after_optin_page_renders_thank_you_continuation() -> Result<(), Box<dyn std::error::Error>>
{
    let (status, body) = get_body(
        "/lander/fake-lander-after-optin?next=http%3A%2F%2F127.0.0.1%3A8088%2Fgo%2Fvisit-3%2F1",
        config_from(&[])?,
    )
    .await?;

    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Fake After-Opt-In Thank You Lander"));
    assert!(body.contains("Fake thank-you step"));
    assert!(body.contains("Continue After Fake Opt-In"));
    Ok(())
}

#[tokio::test]
async fn multi_cta_page_renders_three_distinct_actions() -> Result<(), Box<dyn std::error::Error>> {
    let (status, body) = get_body(
        "/lander/fake-lander-multi-cta?cta1=http%3A%2F%2F127.0.0.1%3A8088%2Fgo%2Fvisit-4%2F1&cta2=http%3A%2F%2F127.0.0.1%3A8088%2Fgo%2Fvisit-4%2F2&cta3=http%3A%2F%2F127.0.0.1%3A8088%2Fgo%2Fvisit-4%2F3",
        config_from(&[])?,
    )
    .await?;

    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("data-continuation=\"cta1\""));
    assert!(body.contains("data-continuation=\"cta2\""));
    assert!(body.contains("data-continuation=\"cta3\""));
    assert!(body.contains("href=\"http://127.0.0.1:8088/go/visit-4/1\""));
    assert!(body.contains("href=\"http://127.0.0.1:8088/go/visit-4/2\""));
    assert!(body.contains("href=\"http://127.0.0.1:8088/go/visit-4/3\""));
    Ok(())
}

#[tokio::test]
async fn missing_continuation_renders_controlled_local_state()
-> Result<(), Box<dyn std::error::Error>> {
    let (status, body) = get_body(
        "/lander/fake-lander-standard-click-through",
        config_from(&[])?,
    )
    .await?;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(body.contains("Continuation unavailable"));
    assert!(body.contains("Missing continuation target: next"));
    assert!(!body.contains("class=\"cta primary\""));
    Ok(())
}

#[tokio::test]
async fn public_continuation_is_blocked_by_default() -> Result<(), Box<dyn std::error::Error>> {
    let (status, body) = get_body(
        "/lander/fake-lander-standard-click-through?next=https%3A%2F%2Fpublic.example%2Fgo",
        config_from(&[])?,
    )
    .await?;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(body.contains("Blocked continuation target in next"));
    assert!(body.contains("public host public.example requires --allow-host"));
    assert!(!body.contains("href=\"https://public.example/go\""));
    Ok(())
}

#[tokio::test]
async fn allowlisted_public_continuation_is_rendered() -> Result<(), Box<dyn std::error::Error>> {
    let (status, body) = get_body(
        "/lander/fake-lander-standard-click-through?next=https%3A%2F%2Fpublic.example%2Fgo",
        config_from(&["--allow-host", "public.example"])?,
    )
    .await?;

    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("href=\"https://public.example/go\""));
    Ok(())
}

#[tokio::test]
async fn unknown_lander_returns_not_found_without_actions() -> Result<(), Box<dyn std::error::Error>>
{
    let (status, body) = get_body(
        "/lander/not-a-preset?next=http%3A%2F%2F127.0.0.1%3A8088%2Fgo%2Fvisit%2F1",
        config_from(&[])?,
    )
    .await?;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body.contains("Fake lander not found"));
    assert!(!body.contains("class=\"cta"));
    Ok(())
}

#[tokio::test]
async fn rendered_continuation_values_are_html_escaped() -> Result<(), Box<dyn std::error::Error>> {
    let (status, body) = get_body(
        "/lander/fake-lander-standard-click-through?next=http%3A%2F%2F127.0.0.1%3A8088%2Fgo%2Fvisit-5%2F1%3Flabel%3D%3Cscript%3E%26x%3D%22quote%22",
        config_from(&[])?,
    )
    .await?;

    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("label=%3Cscript%3E&amp;x=%22quote%22"));
    assert!(!body.contains("<script>"));
    Ok(())
}
