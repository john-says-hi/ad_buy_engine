use std::time::Duration;

use url::Url;

use crate::safety::{SafetyError, SafetyPolicy};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeliveryStatus {
    Pending,
    Succeeded,
    Failed,
    Blocked,
}

impl DeliveryStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostbackDelivery {
    pub status: DeliveryStatus,
    pub response_status: Option<u16>,
    pub response_body: Option<String>,
    pub failure_reason: Option<String>,
}

impl PostbackDelivery {
    pub fn pending() -> Self {
        Self {
            status: DeliveryStatus::Pending,
            response_status: None,
            response_body: None,
            failure_reason: None,
        }
    }

    pub fn failed(reason: impl Into<String>) -> Self {
        Self {
            status: DeliveryStatus::Failed,
            response_status: None,
            response_body: None,
            failure_reason: Some(reason.into()),
        }
    }
}

#[derive(Clone)]
pub struct PostbackClient {
    client: reqwest::Client,
    safety_policy: SafetyPolicy,
}

impl PostbackClient {
    pub fn new(
        safety_policy: SafetyPolicy,
        request_timeout: Duration,
    ) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(request_timeout)
            .build()?;
        Ok(Self {
            client,
            safety_policy,
        })
    }

    pub async fn deliver(&self, url: &Url) -> PostbackDelivery {
        if let Err(error) = self.safety_policy.ensure_url_allowed(url) {
            return blocked_delivery(error);
        }

        match self.client.get(url.clone()).send().await {
            Ok(response) => {
                let status = response.status();
                let status_code = status.as_u16();
                let response_body = match response.text().await {
                    Ok(body) => Some(body_summary(&body)),
                    Err(error) => Some(format!("failed to read response body: {error}")),
                };
                if status.is_success() || status.is_redirection() {
                    PostbackDelivery {
                        status: DeliveryStatus::Succeeded,
                        response_status: Some(status_code),
                        response_body,
                        failure_reason: None,
                    }
                } else {
                    PostbackDelivery {
                        status: DeliveryStatus::Failed,
                        response_status: Some(status_code),
                        response_body,
                        failure_reason: Some(format!("non-success response status {status_code}")),
                    }
                }
            }
            Err(error) => PostbackDelivery::failed(error.to_string()),
        }
    }
}

fn blocked_delivery(error: SafetyError) -> PostbackDelivery {
    PostbackDelivery {
        status: DeliveryStatus::Blocked,
        response_status: None,
        response_body: None,
        failure_reason: Some(error.to_string()),
    }
}

fn body_summary(body: &str) -> String {
    body.chars().take(512).collect()
}
