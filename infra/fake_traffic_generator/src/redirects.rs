use serde::Serialize;
use url::Url;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RedirectStep {
    pub from_url: String,
    pub status: u16,
    pub location: Option<String>,
    pub followed_url: Option<String>,
    pub blocked_reason: Option<String>,
}

pub fn is_redirect_status(status: reqwest::StatusCode) -> bool {
    status.is_redirection()
}

pub fn resolve_location(base_url: &Url, location: &str) -> Result<Url, url::ParseError> {
    match Url::parse(location) {
        Ok(url) => Ok(url),
        Err(_) => base_url.join(location),
    }
}

pub fn extract_go_url(base_url: &Url, target_url: &Url) -> Option<Url> {
    if is_go_url(target_url) {
        return Some(target_url.clone());
    }

    for (_, value) in target_url.query_pairs() {
        if let Some(go_url) = parse_go_candidate(base_url, &value) {
            return Some(go_url);
        }
    }

    None
}

pub fn extract_visit_id(url: &Url) -> Option<String> {
    let mut segments = url.path_segments()?;
    if segments.next()? != "go" {
        return None;
    }
    segments.next().map(ToString::to_string)
}

pub fn is_go_url(url: &Url) -> bool {
    let mut segments = match url.path_segments() {
        Some(segments) => segments,
        None => return false,
    };
    matches!(segments.next(), Some("go")) && segments.next().is_some()
}

fn parse_go_candidate(base_url: &Url, value: &str) -> Option<Url> {
    if let Ok(url) = Url::parse(value)
        && is_go_url(&url)
    {
        return Some(url);
    }
    if is_relative_url_candidate(value)
        && let Ok(url) = base_url.join(value)
        && is_go_url(&url)
    {
        return Some(url);
    }
    let go_start = value.find("/go/")?;
    let suffix = &value[go_start..];
    base_url.join(suffix).ok().filter(is_go_url)
}

fn is_relative_url_candidate(value: &str) -> bool {
    value.starts_with('/') || value.starts_with("./") || value.starts_with("../")
}
