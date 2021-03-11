use strum::IntoEnumIterator;
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug, EnumString, ToString, EnumIter)]
pub enum ReferrerHandling {
    #[strum(serialize = "Do Nothing")]
    DoNothing,
    #[strum(serialize = "Remove Referrers")]
    RemoveAll,
    #[strum(serialize = "Replace Referrers")]
    Replace(ReplaceReferrerList),
}

impl Default for ReferrerHandling {
    fn default() -> Self {
        ReferrerHandling::DoNothing
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ReplaceReferrerList {
    pub name_of_list: String,
    pub percent_of_originals_to_replace: u8,
    pub referrer_list_items: String,
}

impl Default for ReplaceReferrerList {
    fn default() -> Self {
        Self {
            name_of_list: String::new(),
            percent_of_originals_to_replace: 100,
            referrer_list_items: String::new(),
        }
    }
}
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct ReplaceMatchingReferrer {
//     pub name: String,
//     pub original: Url,
//     pub new: Url,
// }
