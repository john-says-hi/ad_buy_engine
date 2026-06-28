pub fn parse_v1_api(scope: &str, full_path: &str, with_id: bool) -> String {
    if with_id {
        let mut x = full_path.replace(format!("/api/v1/{}", scope).as_str(), "");
        x.push_str("/{id}");
        x
    } else {
        full_path
            .replace(format!("/api/v1/{}", scope).as_str(), "")
            .clone()
    }
}

pub fn trim_api_v1(full_api_path: &str) -> String {
    full_api_path.replace("/api/v1", "")
}

pub fn parse_api_v2_url(scope: &str, full_path: &str, with_id: bool) -> String {
    if with_id {
        let mut x = full_path.replace(format!("/api/v2/{}", scope).as_str(), "");
        x.push_str("/{id}");
        x
    } else {
        full_path
            .replace(format!("/api/v2/{}", scope).as_str(), "")
            .clone()
    }
}
