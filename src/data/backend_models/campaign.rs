use crate::data::elements::campaign::Campaign;
#[cfg(feature = "backend")]
use crate::schema::*;
use chrono::NaiveDateTime;

#[cfg_attr(
    feature = "backend",
    derive(Queryable, Insertable, AsChangeset, Identifiable),
    table_name = "campaign_table",
    primary_key("campaign_id")
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CampaignModel {
    pub campaign_id: String,
    pub account_id: String,
    pub campaign_data: String,
    pub last_updated: i64,
}

impl From<Campaign> for CampaignModel {
    fn from(campaign: Campaign) -> Self {
        Self {
            campaign_id: campaign.campaign_id.to_string(),
            account_id: campaign.account_id.to_string(),
            campaign_data: serde_json::to_string(&campaign).expect("G%$#sS"),
            last_updated: campaign.last_updated.timestamp(),
        }
    }
}

impl From<CampaignModel> for Campaign {
    fn from(campaign_model: CampaignModel) -> Self {
        serde_json::from_str(&campaign_model.campaign_data).expect("VG#4rzs")
    }
}
