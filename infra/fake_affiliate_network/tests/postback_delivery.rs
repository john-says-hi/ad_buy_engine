use std::sync::{Arc, Mutex};

use axum::Router;
use axum::extract::Query;
use axum::routing::get;
use fake_affiliate_network::postback::{DeliveryStatus, PostbackClient};
use fake_affiliate_network::safety::SafetyPolicy;
use tokio::sync::oneshot;
use url::Url;

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

#[tokio::test]
async fn successful_loopback_callback_records_response_status()
-> Result<(), Box<dyn std::error::Error>> {
    let received = Arc::new(Mutex::new(Vec::<String>::new()));
    let handler_received = Arc::clone(&received);
    let server = spawn_server(Router::new().route(
        "/postback",
        get(
            move |Query(params): Query<std::collections::HashMap<String, String>>| {
                let handler_received = Arc::clone(&handler_received);
                async move {
                    if let Ok(mut items) = handler_received.lock()
                        && let Some(click_id) = params.get("cid")
                    {
                        items.push(click_id.clone());
                    }
                    "ok"
                }
            },
        ),
    ))
    .await?;
    let client = PostbackClient::new(
        SafetyPolicy::new(Vec::new(), false),
        std::time::Duration::from_secs(5),
    )?;
    let url = Url::parse(&format!("{}/postback?cid=visit-123", server.base_url))?;

    let delivery = client.deliver(&url).await;

    assert_eq!(delivery.status, DeliveryStatus::Succeeded);
    assert_eq!(delivery.response_status, Some(200));
    let captured = received
        .lock()
        .map(|items| items.clone())
        .unwrap_or_default();
    assert_eq!(captured, vec!["visit-123".to_string()]);
    Ok(())
}

#[tokio::test]
async fn public_callbacks_are_blocked_without_allowlist() -> Result<(), Box<dyn std::error::Error>>
{
    let client = PostbackClient::new(
        SafetyPolicy::new(Vec::new(), false),
        std::time::Duration::from_secs(5),
    )?;
    let url = Url::parse("https://public.example/postback?cid=visit-123")?;

    let delivery = client.deliver(&url).await;

    assert_eq!(delivery.status, DeliveryStatus::Blocked);
    assert!(
        delivery
            .failure_reason
            .unwrap_or_default()
            .contains("public host public.example")
    );
    Ok(())
}
