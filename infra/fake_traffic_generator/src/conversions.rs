use url::Url;

use crate::config::RunConfig;
use crate::profiles::conversion_rng;

pub fn should_send_conversion(config: &RunConfig, session_index: u64) -> bool {
    if !config.conversions_enabled() {
        return false;
    }
    let mut rng = conversion_rng(config.seed, session_index);
    rng.chance(config.conversion_rate)
}

pub fn postback_url(config: &RunConfig, visit_id: &str, session_index: u64) -> Option<Url> {
    let conversion_type = config.conversion_type.postback_value()?;
    let mut url = config.campaign_url.clone();
    url.set_path("/postback");
    url.set_query(None);
    url.set_fragment(None);
    {
        let mut pairs = url.query_pairs_mut();
        pairs.append_pair("cid", visit_id);
        pairs.append_pair("type", conversion_type);
        pairs.append_pair(
            "eventid",
            &format!("abe-fake-{}-{session_index}", config.seed),
        );
        if conversion_type == "Sale" {
            pairs.append_pair("payout", "1.00");
        }
    }
    Some(url)
}
