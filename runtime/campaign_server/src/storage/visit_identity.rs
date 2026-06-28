pub fn unique_visit_key(id: &str, ip_address: Option<&str>, user_agent: Option<&str>) -> String {
    let ip_address = ip_address.unwrap_or_default();
    let user_agent = user_agent.unwrap_or_default();
    if ip_address.is_empty() && user_agent.is_empty() {
        return id.to_string();
    }
    format!("{ip_address}|{user_agent}")
}
