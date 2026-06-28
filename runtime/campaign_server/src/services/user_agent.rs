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

#[cfg(test)]
mod tests {
    use super::{detect_browser, detect_device_type, detect_operating_system};

    #[test]
    fn detects_common_browser_device_and_os_values() {
        let chrome_linux = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
            (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36";
        assert_eq!(detect_browser(chrome_linux), "Chrome");
        assert_eq!(detect_operating_system(chrome_linux), "Linux");
        assert_eq!(detect_device_type(chrome_linux), "Desktop");

        let mobile_safari = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_5 like Mac OS X) \
            AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Mobile/15E148 Safari/604.1";
        assert_eq!(detect_browser(mobile_safari), "Safari");
        assert_eq!(detect_operating_system(mobile_safari), "iOS");
        assert_eq!(detect_device_type(mobile_safari), "Mobile");
    }
}
