use crate::route::Route;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CreateFormDefinition {
    pub route: Route,
    pub modal_id: &'static str,
    pub title: &'static str,
    pub sections: &'static [CreateFormSection],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CreateFormSection {
    pub title: Option<&'static str>,
    pub fields: &'static [CreateFormField],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CreateFormField {
    Text {
        label: &'static str,
        placeholder: &'static str,
        value: &'static str,
    },
    Number {
        label: &'static str,
        placeholder: &'static str,
        value: &'static str,
    },
    Select {
        label: &'static str,
        selected: &'static str,
        options: &'static [&'static str],
    },
    TextArea {
        label: &'static str,
        placeholder: &'static str,
        value: &'static str,
        rows: u8,
    },
    Toggle {
        label: &'static str,
        checked: bool,
    },
    RadioGroup {
        label: &'static str,
        selected: &'static str,
        options: &'static [&'static str],
    },
    TokenTable {
        label: &'static str,
        rows: &'static [TokenTableRow],
    },
    GeneratedValue {
        label: &'static str,
        value: &'static str,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TokenTableRow {
    pub name: &'static str,
    pub token: &'static str,
}

impl CreateFormDefinition {
    pub const fn for_route(route: Route) -> Option<Self> {
        match route.render_route() {
            Route::Campaigns => Some(Self {
                route: Route::Campaigns,
                modal_id: "campaigns",
                title: "Create Campaign",
                sections: &CAMPAIGN_SECTIONS,
            }),
            Route::Offers => Some(Self {
                route: Route::Offers,
                modal_id: "offer",
                title: "New Offer",
                sections: &OFFER_SECTIONS,
            }),
            Route::Landers => Some(Self {
                route: Route::Landers,
                modal_id: "landing-pages",
                title: "New Lander",
                sections: &LANDER_SECTIONS,
            }),
            Route::Funnels => Some(Self {
                route: Route::Funnels,
                modal_id: "funnels",
                title: "Create Funnel",
                sections: &FUNNEL_SECTIONS,
            }),
            Route::TrafficSources => Some(Self {
                route: Route::TrafficSources,
                modal_id: "traffic-sources",
                title: "New Traffic Source",
                sections: &TRAFFIC_SOURCE_SECTIONS,
            }),
            Route::OfferSources => Some(Self {
                route: Route::OfferSources,
                modal_id: "offer-sources",
                title: "New Offer Source",
                sections: &OFFER_SOURCE_SECTIONS,
            }),
            _ => None,
        }
    }

    pub fn contains_field_label(self, expected_label: &str) -> bool {
        self.sections.iter().any(|section| {
            section.fields.iter().any(|field| {
                field.label() == expected_label
                    || field.table_rows().is_some_and(|rows| {
                        rows.iter()
                            .any(|row| row.name == expected_label || row.token == expected_label)
                    })
            })
        })
    }
}

impl CreateFormField {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Text { label, .. }
            | Self::Number { label, .. }
            | Self::Select { label, .. }
            | Self::TextArea { label, .. }
            | Self::Toggle { label, .. }
            | Self::RadioGroup { label, .. }
            | Self::TokenTable { label, .. }
            | Self::GeneratedValue { label, .. } => label,
        }
    }

    pub const fn table_rows(self) -> Option<&'static [TokenTableRow]> {
        match self {
            Self::TokenTable { rows, .. } => Some(rows),
            _ => None,
        }
    }
}

const COUNTRY_OPTIONS: &[&str] = &["Global", "United States", "Canada", "United Kingdom"];
const COST_MODEL_OPTIONS: &[&str] = &["CPC", "CPA", "CPM", "RevShare", "Not Tracked"];
const CURRENCY_OPTIONS: &[&str] = &["USD", "CAD", "EUR", "GBP"];
const DESTINATION_OPTIONS: &[&str] = &["Funnel", "Sequence"];
const FUNNEL_OPTIONS: &[&str] = &["Select Funnel"];
const OFFER_SOURCE_OPTIONS: &[&str] = &["Select Offer Source"];
const PAYOUT_TYPE_OPTIONS: &[&str] = &["Auto", "Fixed", "Percentage", "None"];
const REFERRER_OPTIONS: &[&str] = &["Do Nothing", "Hide Referrer", "Pass Referrer"];
const TRACKING_DOMAIN_OPTIONS: &[&str] = &["Main Tracking Domain"];
const TRACKING_METHOD_OPTIONS: &[&str] = &["Postback URL", "Tracking Pixel"];
const TRAFFIC_SOURCE_OPTIONS: &[&str] = &["Select Traffic Source"];

const CAMPAIGN_SECTIONS: [CreateFormSection; 3] = [
    CreateFormSection {
        title: None,
        fields: &CAMPAIGN_CORE_FIELDS,
    },
    CreateFormSection {
        title: Some("Setup Campaign Destination"),
        fields: &CAMPAIGN_DESTINATION_FIELDS,
    },
    CreateFormSection {
        title: None,
        fields: &CAMPAIGN_NOTES_FIELDS,
    },
];

const CAMPAIGN_CORE_FIELDS: [CreateFormField; 5] = [
    CreateFormField::Select {
        label: "Traffic Source",
        selected: "Select Traffic Source",
        options: TRAFFIC_SOURCE_OPTIONS,
    },
    CreateFormField::Select {
        label: "Country ",
        selected: "Global",
        options: COUNTRY_OPTIONS,
    },
    CreateFormField::Text {
        label: "Name",
        placeholder: "Name",
        value: "New Campaign",
    },
    CreateFormField::Select {
        label: "Cost Model",
        selected: "CPC",
        options: COST_MODEL_OPTIONS,
    },
    CreateFormField::Number {
        label: "Cost Value in USD",
        placeholder: "0",
        value: "0",
    },
];

const CAMPAIGN_DESTINATION_FIELDS: [CreateFormField; 2] = [
    CreateFormField::RadioGroup {
        label: "Destination Type",
        selected: "Funnel",
        options: DESTINATION_OPTIONS,
    },
    CreateFormField::Select {
        label: "Select Funnel",
        selected: "Select Funnel",
        options: FUNNEL_OPTIONS,
    },
];

const CAMPAIGN_NOTES_FIELDS: [CreateFormField; 1] = [CreateFormField::TextArea {
    label: "Notes",
    placeholder: "",
    value: "",
    rows: 4,
}];

const OFFER_SECTIONS: [CreateFormSection; 3] = [
    CreateFormSection {
        title: None,
        fields: &OFFER_CORE_FIELDS,
    },
    CreateFormSection {
        title: Some("Offer URL"),
        fields: &OFFER_URL_FIELDS,
    },
    CreateFormSection {
        title: None,
        fields: &OFFER_NOTES_FIELDS,
    },
];

const OFFER_CORE_FIELDS: [CreateFormField; 6] = [
    CreateFormField::Select {
        label: "Link Offer Source:",
        selected: "Select Offer Source",
        options: OFFER_SOURCE_OPTIONS,
    },
    CreateFormField::Select {
        label: "Country ",
        selected: "Global",
        options: COUNTRY_OPTIONS,
    },
    CreateFormField::Text {
        label: "Offer Name:",
        placeholder: "Name",
        value: "",
    },
    CreateFormField::Text {
        label: "Tags",
        placeholder: "Add tags",
        value: "",
    },
    CreateFormField::Select {
        label: "Payout Type",
        selected: "Auto",
        options: PAYOUT_TYPE_OPTIONS,
    },
    CreateFormField::Select {
        label: "Payout Currency",
        selected: "USD",
        options: CURRENCY_OPTIONS,
    },
];

const OFFER_URL_FIELDS: [CreateFormField; 2] = [
    CreateFormField::Text {
        label: "Offer URL",
        placeholder: "https://example.com/offer?clickid={clickid}",
        value: "",
    },
    CreateFormField::GeneratedValue {
        label: "Offer URL Tokens",
        value: "clickid, payout, conversion_id",
    },
];

const OFFER_NOTES_FIELDS: [CreateFormField; 1] = [CreateFormField::TextArea {
    label: "Notes",
    placeholder: "",
    value: "",
    rows: 4,
}];

const LANDER_SECTIONS: [CreateFormSection; 3] = [
    CreateFormSection {
        title: None,
        fields: &LANDER_CORE_FIELDS,
    },
    CreateFormSection {
        title: Some("Landing Page URLs"),
        fields: &LANDER_URL_FIELDS,
    },
    CreateFormSection {
        title: None,
        fields: &LANDER_NOTES_FIELDS,
    },
];

const LANDER_CORE_FIELDS: [CreateFormField; 4] = [
    CreateFormField::Select {
        label: "Country ",
        selected: "Global",
        options: COUNTRY_OPTIONS,
    },
    CreateFormField::Text {
        label: "Lander Name:",
        placeholder: "Name",
        value: "",
    },
    CreateFormField::Text {
        label: "Tags",
        placeholder: "Add tags",
        value: "",
    },
    CreateFormField::Number {
        label: "Number of CTAs:",
        placeholder: "1",
        value: "1",
    },
];

const LANDER_URL_FIELDS: [CreateFormField; 4] = [
    CreateFormField::Text {
        label: "Lander URL",
        placeholder: "https://example.com/lander?clickid={clickid}",
        value: "",
    },
    CreateFormField::GeneratedValue {
        label: "Lander URL Tokens",
        value: "clickid, campaign, traffic_source",
    },
    CreateFormField::Select {
        label: "Tracking Domain",
        selected: "Main Tracking Domain",
        options: TRACKING_DOMAIN_OPTIONS,
    },
    CreateFormField::GeneratedValue {
        label: "Landing Page Click URL",
        value: "https://tracking-domain.example/click/1",
    },
];

const LANDER_NOTES_FIELDS: [CreateFormField; 1] = [CreateFormField::TextArea {
    label: "Notes",
    placeholder: "",
    value: "",
    rows: 4,
}];

const FUNNEL_SECTIONS: [CreateFormSection; 3] = [
    CreateFormSection {
        title: None,
        fields: &FUNNEL_CORE_FIELDS,
    },
    CreateFormSection {
        title: Some("Sequences"),
        fields: &FUNNEL_SEQUENCE_FIELDS,
    },
    CreateFormSection {
        title: None,
        fields: &FUNNEL_NOTES_FIELDS,
    },
];

const FUNNEL_CORE_FIELDS: [CreateFormField; 3] = [
    CreateFormField::Text {
        label: "Funnel Name",
        placeholder: "Name",
        value: "New Funnel",
    },
    CreateFormField::Select {
        label: "Country ",
        selected: "Global",
        options: COUNTRY_OPTIONS,
    },
    CreateFormField::Select {
        label: "Referrer Handling",
        selected: "Do Nothing",
        options: REFERRER_OPTIONS,
    },
];

const FUNNEL_SEQUENCE_FIELDS: [CreateFormField; 3] = [
    CreateFormField::GeneratedValue {
        label: "Conditional Sequences",
        value: "No conditional sequences",
    },
    CreateFormField::GeneratedValue {
        label: "Default Sequences",
        value: "No default sequences",
    },
    CreateFormField::GeneratedValue {
        label: "Active Element",
        value: "Global - New Funnel",
    },
];

const FUNNEL_NOTES_FIELDS: [CreateFormField; 1] = [CreateFormField::TextArea {
    label: "Notes",
    placeholder: "",
    value: "",
    rows: 4,
}];

const TRAFFIC_SOURCE_SECTIONS: [CreateFormSection; 3] = [
    CreateFormSection {
        title: None,
        fields: &TRAFFIC_SOURCE_CORE_FIELDS,
    },
    CreateFormSection {
        title: Some("Tracking Parameters"),
        fields: &TRAFFIC_SOURCE_TRACKING_FIELDS,
    },
    CreateFormSection {
        title: None,
        fields: &TRAFFIC_SOURCE_NOTES_FIELDS,
    },
];

const TRAFFIC_SOURCE_CORE_FIELDS: [CreateFormField; 2] = [
    CreateFormField::Text {
        label: "Traffic Source Name:",
        placeholder: "Name",
        value: "",
    },
    CreateFormField::Select {
        label: "Cost Currency",
        selected: "USD",
        options: CURRENCY_OPTIONS,
    },
];

const TRAFFIC_SOURCE_TRACKING_FIELDS: [CreateFormField; 4] = [
    CreateFormField::TokenTable {
        label: "URL Parameters",
        rows: &TRAFFIC_SOURCE_TOKEN_ROWS,
    },
    CreateFormField::Toggle {
        label: "Traffic Source Postback URL",
        checked: false,
    },
    CreateFormField::Text {
        label: "Traffic Source Postback URL Generator",
        placeholder: "https://traffic-source.example/postback?clickid={clickid}",
        value: "",
    },
    CreateFormField::GeneratedValue {
        label: "Traffic Source Postback URL Tokens",
        value: "clickid, cost, custom",
    },
];

const TRAFFIC_SOURCE_TOKEN_ROWS: [TokenTableRow; 3] = [
    TokenTableRow {
        name: "External ID",
        token: "{external_id}",
    },
    TokenTableRow {
        name: "Cost",
        token: "{cost}",
    },
    TokenTableRow {
        name: "Custom Parameters",
        token: "{custom}",
    },
];

const TRAFFIC_SOURCE_NOTES_FIELDS: [CreateFormField; 1] = [CreateFormField::TextArea {
    label: "Notes",
    placeholder: "",
    value: "",
    rows: 4,
}];

const OFFER_SOURCE_SECTIONS: [CreateFormSection; 4] = [
    CreateFormSection {
        title: None,
        fields: &OFFER_SOURCE_CORE_FIELDS,
    },
    CreateFormSection {
        title: None,
        fields: &OFFER_SOURCE_TRACKING_FIELDS,
    },
    CreateFormSection {
        title: Some("Postback Options"),
        fields: &OFFER_SOURCE_POSTBACK_FIELDS,
    },
    CreateFormSection {
        title: None,
        fields: &OFFER_SOURCE_NOTES_FIELDS,
    },
];

const OFFER_SOURCE_CORE_FIELDS: [CreateFormField; 1] = [CreateFormField::Text {
    label: "Name of Source:",
    placeholder: "Name",
    value: "",
}];

const OFFER_SOURCE_TRACKING_FIELDS: [CreateFormField; 5] = [
    CreateFormField::TokenTable {
        label: "Tracking Parameters",
        rows: &OFFER_SOURCE_TOKEN_ROWS,
    },
    CreateFormField::Select {
        label: "Tracking Domain",
        selected: "Main Tracking Domain",
        options: TRACKING_DOMAIN_OPTIONS,
    },
    CreateFormField::Select {
        label: "Tracking Method",
        selected: "Postback URL",
        options: TRACKING_METHOD_OPTIONS,
    },
    CreateFormField::Toggle {
        label: "Include More Parameters",
        checked: false,
    },
    CreateFormField::TextArea {
        label: "Postback URL",
        placeholder: "",
        value: "https://tracking-domain.example/postback?cid={click_id}&payout={payout}",
        rows: 4,
    },
];

const OFFER_SOURCE_TOKEN_ROWS: [TokenTableRow; 4] = [
    TokenTableRow {
        name: "Click ID",
        token: "{click_id}",
    },
    TokenTableRow {
        name: "Payout",
        token: "{payout}",
    },
    TokenTableRow {
        name: "Conversion ID",
        token: "{conversion_id}",
    },
    TokenTableRow {
        name: "Custom Events",
        token: "Select Events",
    },
];

const OFFER_SOURCE_POSTBACK_FIELDS: [CreateFormField; 5] = [
    CreateFormField::Select {
        label: "Payout Currency",
        selected: "USD",
        options: CURRENCY_OPTIONS,
    },
    CreateFormField::Toggle {
        label: "Append Click ID",
        checked: false,
    },
    CreateFormField::Toggle {
        label: "Accept Duplicate Postbacks",
        checked: false,
    },
    CreateFormField::Toggle {
        label: "Whitelist Postback URL IPs",
        checked: false,
    },
    CreateFormField::Select {
        label: "Referrer Handling",
        selected: "Do Nothing",
        options: REFERRER_OPTIONS,
    },
];

const OFFER_SOURCE_NOTES_FIELDS: [CreateFormField; 1] = [CreateFormField::TextArea {
    label: "Notes",
    placeholder: "",
    value: "",
    rows: 4,
}];
