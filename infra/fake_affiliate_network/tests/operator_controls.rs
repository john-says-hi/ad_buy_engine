use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use axum::routing::get;
use clap::Parser;
use fake_affiliate_network::build_router;
use fake_affiliate_network::cli::Cli;
use fake_affiliate_network::config::RunConfig;
use tokio::sync::oneshot;
use tower::ServiceExt;

struct TestServer {
    base_url: String,
    shutdown: Option<oneshot::Sender<()>>,
}

impl Drop for TestServer {
    fn drop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _send_result = shutdown.send(());
        }
    }
}

async fn spawn_server(router: Router) -> Result<TestServer, Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let address = listener.local_addr()?;
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    tokio::spawn(async move {
        let server = axum::serve(listener, router).with_graceful_shutdown(async {
            let _result = shutdown_rx.await;
        });
        let _result = server.await;
    });
    Ok(TestServer {
        base_url: format!("http://{address}"),
        shutdown: Some(shutdown_tx),
    })
}

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
async fn valid_settings_and_sample_lead_postback_are_visible()
-> Result<(), Box<dyn std::error::Error>> {
    let callback = spawn_server(Router::new().route("/postback", get(|| async { "ok" }))).await?;
    let template = format!(
        "{}/postback?cid={{click_id}}&type={{event_type}}&payout={{payout}}&currency={{currency}}&status={{status}}&txid={{transaction_id}}",
        callback.base_url
    );
    let app = build_router(config_from(&["--postback-template", &template])?)?;

    let settings_body = format!(
        "postback_template={}&lead_threshold=2&sale_threshold=3",
        urlencoding::encode(&template)
    );
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/settings")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(settings_body))?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/sample")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(
                    "event_type=Lead&tracking_identifier=sample-visit",
                ))?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);

    let dashboard = app
        .oneshot(Request::builder().uri("/").body(Body::empty())?)
        .await?;
    let body = response_text(dashboard).await?;
    assert!(body.contains("sample-visit"));
    assert!(body.contains("succeeded"));
    assert!(body.contains("Lead"));
    Ok(())
}

#[tokio::test]
async fn invalid_settings_keep_last_valid_settings_active() -> Result<(), Box<dyn std::error::Error>>
{
    let app = build_router(config_from(&[])?)?;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/settings")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(
                    "postback_template=https%3A%2F%2Fpublic.example%2Fpostback%3Fcid%3D%7Bclick_id%7D&lead_threshold=0&sale_threshold=3",
                ))?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::OK);
    let body = response_text(response).await?;
    assert!(body.contains("lead-threshold"));

    let dashboard = app
        .oneshot(Request::builder().uri("/").body(Body::empty())?)
        .await?;
    let body = response_text(dashboard).await?;
    assert!(body.contains("127.0.0.1:8088/postback"));
    Ok(())
}
