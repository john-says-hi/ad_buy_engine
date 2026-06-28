use ad_buy_engine_domain::VisitEnrichment;

pub fn detect_browser(user_agent: &str) -> String {
    if user_agent.contains("Firefox") {
        "Firefox".to_string()
    } else if user_agent.contains("Edg/") {
        "Edge".to_string()
    } else if user_agent.contains("Chrome") || user_agent.contains("CriOS") {
        "Chrome".to_string()
    } else if user_agent.contains("Safari") {
        "Safari".to_string()
    } else {
        "Other".to_string()
    }
}

pub fn detect_browser_version(user_agent: &str) -> Option<String> {
    if user_agent.contains("Firefox/") {
        token_after(user_agent, "Firefox/")
    } else if user_agent.contains("Edg/") {
        token_after(user_agent, "Edg/")
    } else if user_agent.contains("Chrome/") {
        token_after(user_agent, "Chrome/")
    } else if user_agent.contains("CriOS/") {
        token_after(user_agent, "CriOS/")
    } else if user_agent.contains("Safari/") {
        token_after(user_agent, "Version/")
    } else {
        None
    }
}

pub fn detect_operating_system(user_agent: &str) -> String {
    if user_agent.contains("iPhone") || user_agent.contains("iPad") {
        "iOS".to_string()
    } else if user_agent.contains("Android") {
        "Android".to_string()
    } else if user_agent.contains("Windows") {
        "Windows".to_string()
    } else if user_agent.contains("Mac OS X") {
        "macOS".to_string()
    } else if user_agent.contains("Linux") {
        "Linux".to_string()
    } else {
        "Other".to_string()
    }
}

pub fn detect_operating_system_version(user_agent: &str) -> Option<String> {
    if let Some(version) = token_after(user_agent, "Windows NT ") {
        return Some(version);
    }
    if let Some(version) = token_after(user_agent, "CPU iPhone OS ") {
        return Some(version.replace('_', "."));
    }
    if let Some(version) = token_after(user_agent, "CPU OS ") {
        return Some(version.replace('_', "."));
    }
    token_after(user_agent, "Android ")
}

pub fn detect_device_type(user_agent: &str) -> String {
    if user_agent.contains("Tablet") || user_agent.contains("iPad") {
        "Tablet".to_string()
    } else if user_agent.contains("Mobile")
        || user_agent.contains("iPhone")
        || user_agent.contains("Android")
    {
        "Mobile".to_string()
    } else {
        "Desktop".to_string()
    }
}

pub fn detect_device_brand(user_agent: &str) -> Option<String> {
    if user_agent.contains("iPhone")
        || user_agent.contains("iPad")
        || user_agent.contains("Macintosh")
    {
        Some("Apple".to_string())
    } else if user_agent.contains("Pixel") {
        Some("Google".to_string())
    } else if user_agent.contains("SAMSUNG") || user_agent.contains("SM-") {
        Some("Samsung".to_string())
    } else {
        None
    }
}

pub fn detect_device_model(user_agent: &str) -> Option<String> {
    if user_agent.contains("iPhone") {
        return Some("iPhone".to_string());
    }
    if user_agent.contains("iPad") {
        return Some("iPad".to_string());
    }
    android_model(user_agent)
}

pub fn user_agent_enrichment(user_agent: Option<&str>) -> VisitEnrichment {
    let Some(user_agent) = user_agent else {
        return VisitEnrichment::default();
    };
    VisitEnrichment {
        browser: Some(detect_browser(user_agent)),
        browser_version: detect_browser_version(user_agent),
        operating_system: Some(detect_operating_system(user_agent)),
        operating_system_version: detect_operating_system_version(user_agent),
        device_type: Some(detect_device_type(user_agent)),
        device_brand: detect_device_brand(user_agent),
        device_model: detect_device_model(user_agent),
        ..VisitEnrichment::default()
    }
}

fn token_after(user_agent: &str, marker: &str) -> Option<String> {
    let start = user_agent.find(marker)? + marker.len();
    let value = user_agent[start..]
        .split(|character: char| character.is_whitespace() || matches!(character, ';' | ')' | '('))
        .next()
        .unwrap_or_default()
        .trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn android_model(user_agent: &str) -> Option<String> {
    let android_start = user_agent.find("Android ")?;
    let after_android = &user_agent[android_start..];
    let model = after_android
        .split(';')
        .map(str::trim)
        .find(|part| {
            !part.starts_with("Android ")
                && !part.eq_ignore_ascii_case("Mobile")
                && !part.eq_ignore_ascii_case("wv")
                && !part.is_empty()
        })?
        .trim_end_matches(')');
    if model.is_empty() {
        None
    } else {
        Some(model.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        detect_browser, detect_browser_version, detect_device_brand, detect_device_model,
        detect_device_type, detect_operating_system, detect_operating_system_version,
    };

    #[test]
    fn detects_common_browser_device_and_os_values() {
        let chrome_linux = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
            (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36";
        assert_eq!(detect_browser(chrome_linux), "Chrome");
        assert_eq!(
            detect_browser_version(chrome_linux),
            Some("125.0.0.0".to_string())
        );
        assert_eq!(detect_operating_system(chrome_linux), "Linux");
        assert_eq!(detect_device_type(chrome_linux), "Desktop");

        let mobile_safari = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_5 like Mac OS X) \
            AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Mobile/15E148 Safari/604.1";
        assert_eq!(detect_browser(mobile_safari), "Safari");
        assert_eq!(
            detect_browser_version(mobile_safari),
            Some("17.5".to_string())
        );
        assert_eq!(detect_operating_system(mobile_safari), "iOS");
        assert_eq!(
            detect_operating_system_version(mobile_safari),
            Some("17.5".to_string())
        );
        assert_eq!(detect_device_type(mobile_safari), "Mobile");
        assert_eq!(
            detect_device_brand(mobile_safari),
            Some("Apple".to_string())
        );
        assert_eq!(
            detect_device_model(mobile_safari),
            Some("iPhone".to_string())
        );
    }
}
