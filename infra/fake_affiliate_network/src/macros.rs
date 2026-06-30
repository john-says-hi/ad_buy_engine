use thiserror::Error;
use url::Url;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PostbackMacros<'a> {
    pub click_id: &'a str,
    pub event_type: &'a str,
    pub payout: &'a str,
    pub currency: &'a str,
    pub status: &'a str,
    pub transaction_id: &'a str,
}

#[derive(Debug, Error)]
pub enum MacroError {
    #[error("postback template must not be empty")]
    EmptyTemplate,
    #[error("postback template contains an unknown or unclosed macro")]
    UnknownMacro,
    #[error("postback template must render to a valid HTTP or HTTPS URL: {0}")]
    InvalidUrl(String),
}

pub fn render_postback_url(template: &str, macros: &PostbackMacros<'_>) -> Result<Url, MacroError> {
    let trimmed = template.trim();
    if trimmed.is_empty() {
        return Err(MacroError::EmptyTemplate);
    }

    let rendered = trimmed
        .replace("{click_id}", &encode(macros.click_id))
        .replace("{subid}", &encode(macros.click_id))
        .replace("{event_type}", &encode(macros.event_type))
        .replace("{payout}", &encode(macros.payout))
        .replace("{currency}", &encode(macros.currency))
        .replace("{status}", &encode(macros.status))
        .replace("{transaction_id}", &encode(macros.transaction_id));
    if rendered.contains('{') || rendered.contains('}') {
        return Err(MacroError::UnknownMacro);
    }

    let url = Url::parse(&rendered).map_err(|error| MacroError::InvalidUrl(error.to_string()))?;
    if !matches!(url.scheme(), "http" | "https") {
        return Err(MacroError::InvalidUrl(
            "scheme must be http or https".to_string(),
        ));
    }
    Ok(url)
}

fn encode(value: &str) -> String {
    urlencoding::encode(value).into_owned()
}
