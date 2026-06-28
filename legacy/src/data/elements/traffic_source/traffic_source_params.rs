#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CostParameter {
    pub parameter: String,
    pub placeholder: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExternalIDParameter {
    pub parameter: String,
    pub placeholder: String,
}

impl Default for ExternalIDParameter {
    fn default() -> Self {
        Self {
            parameter: "".to_string(),
            placeholder: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomParameter {
    pub name: String,
    pub parameter: String,
    pub placeholder: String,
    pub is_tracked: bool,
}

impl Default for CustomParameter {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            parameter: "".to_string(),
            placeholder: "".to_string(),
            is_tracked: false,
        }
    }
}
