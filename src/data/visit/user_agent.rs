#[cfg(feature = "use-ua-parser")]
use user_agent_parser::UserAgentParser;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserAgentData {
    pub user_agent_string: String,
    pub cpu_type: String,
    pub device_name: String,
    pub device_brand: String,
    pub device_model: String,
    pub rendering_engine_name: String,
    pub rendering_engine_major: String,
    pub rendering_engine_minor: String,
    pub rendering_engine_patch: String,
    pub os_name: String,
    pub os_major: String,
    pub os_minor: String,
    pub os_patch: String,
    pub os_patch_minor: String,
    pub browser_name: String,
    pub browser_major: String,
    pub browser_minor: String,
    pub browser_patch: String,
}

impl UserAgentData {
    #[cfg(feature = "use-ua-parser")]
    pub fn new(ua_string: String) -> Self {
        let parser = UserAgentParser::from_path("regexes.yaml").unwrap();

        let cpu = parser.parse_cpu(&ua_string);

        let browser = parser.parse_product(&ua_string);

        let rendering_engine = parser.parse_engine(&ua_string);

        let os = parser.parse_os(&ua_string);

        let device = parser.parse_device(&ua_string);

        let cpu_type = if let Some(x) = cpu.architecture {
            String::from(x)
        } else {
            "".to_string()
        };
        let device_name = if let Some(x) = device.name {
            String::from(x)
        } else {
            "".to_string()
        };
        let device_brand = if let Some(x) = device.brand {
            String::from(x)
        } else {
            "".to_string()
        };
        let device_model = if let Some(x) = device.model {
            String::from(x)
        } else {
            "".to_string()
        };
        let rendering_engine_name = if let Some(x) = rendering_engine.name {
            String::from(x)
        } else {
            "".to_string()
        };
        let rendering_engine_major = if let Some(x) = rendering_engine.major {
            String::from(x)
        } else {
            "".to_string()
        };
        let rendering_engine_minor = if let Some(x) = rendering_engine.minor {
            String::from(x)
        } else {
            "".to_string()
        };
        let rendering_engine_patch = if let Some(x) = rendering_engine.patch {
            String::from(x)
        } else {
            "".to_string()
        };
        let os_name = if let Some(x) = os.name {
            String::from(x)
        } else {
            "".to_string()
        };
        let os_major = if let Some(x) = os.major {
            String::from(x)
        } else {
            "".to_string()
        };
        let os_minor = if let Some(x) = os.minor {
            String::from(x)
        } else {
            "".to_string()
        };
        let os_patch = if let Some(x) = os.patch {
            String::from(x)
        } else {
            "".to_string()
        };
        let os_patch_minor = if let Some(x) = os.patch_minor {
            String::from(x)
        } else {
            "".to_string()
        };
        let browser_name = if let Some(x) = browser.name {
            String::from(x)
        } else {
            "".to_string()
        };
        let browser_major = if let Some(x) = browser.major {
            String::from(x)
        } else {
            "".to_string()
        };
        let browser_minor = if let Some(x) = browser.minor {
            String::from(x)
        } else {
            "".to_string()
        };
        let browser_patch = if let Some(x) = browser.patch {
            String::from(x)
        } else {
            "".to_string()
        };

        Self {
            user_agent_string: ua_string,
            cpu_type,
            device_name,
            device_brand,
            device_model,
            rendering_engine_name,
            rendering_engine_major,
            rendering_engine_minor,
            rendering_engine_patch,
            os_name,
            os_major,
            os_minor,
            os_patch,
            os_patch_minor,
            browser_name,
            browser_major,
            browser_minor,
            browser_patch,
        }
    }
}
