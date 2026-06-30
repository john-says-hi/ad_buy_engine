use ad_buy_engine_domain::{
    FakeAffiliateOfferKind, fake_affiliate_catalog, fake_affiliate_offer_url,
};

use crate::postback::DeliveryStatus;
use crate::state::{ClickOutcome, ConversionOrigin, NetworkSnapshot, visible_delivery_label};

pub fn dashboard(base_url: &str, snapshot: &NetworkSnapshot, message: Option<&str>) -> String {
    let mut html = page_start("Fake Affiliate Network");
    html.push_str("<main><h1>Fake Affiliate Network</h1>");
    if let Some(message) = message {
        html.push_str(&format!("<p class=\"notice\">{}</p>", escape_html(message)));
    }
    html.push_str(
        "<section><h2>Postback Settings</h2>\
         <form method=\"post\" action=\"/settings\">",
    );
    html.push_str(&format!(
        "<label>Postback URL Template <input name=\"postback_template\" value=\"{}\"></label>",
        escape_html(&snapshot.settings.postback_template)
    ));
    html.push_str(&format!(
        "<label>Lead Threshold <input name=\"lead_threshold\" type=\"number\" min=\"1\" value=\"{}\"></label>",
        snapshot.settings.lead_threshold
    ));
    html.push_str(&format!(
        "<label>Sale Threshold <input name=\"sale_threshold\" type=\"number\" min=\"1\" value=\"{}\"></label>",
        snapshot.settings.sale_threshold
    ));
    html.push_str("<button type=\"submit\">Save</button></form>");
    html.push_str(
        "<form method=\"post\" action=\"/sample\" class=\"inline-actions\">\
         <input name=\"tracking_identifier\" value=\"sample-click-1\">\
         <button name=\"event_type\" value=\"Lead\" type=\"submit\">Send Sample Lead</button>\
         <button name=\"event_type\" value=\"Sale\" type=\"submit\">Send Sample Sale</button>\
         </form></section>",
    );

    html.push_str("<section><h2>Offers</h2><div class=\"offers\">");
    for offer in fake_affiliate_catalog() {
        let link = fake_affiliate_offer_url(base_url, offer.id);
        html.push_str("<article>");
        html.push_str(&format!(
            "<h3>{}</h3><p>{}</p>",
            escape_html(offer.name),
            escape_html(offer.display_copy)
        ));
        html.push_str(&format!(
            "<dl><dt>Event</dt><dd>{}</dd><dt>Payout</dt><dd>{:.2} {}</dd>\
             <dt>Default threshold</dt><dd>{}</dd><dt>Vertical</dt><dd>{}</dd></dl>",
            escape_html(offer.event_type()),
            offer.payout_value,
            escape_html(offer.currency),
            offer.default_threshold,
            escape_html(offer.vertical)
        ));
        html.push_str(&format!(
            "<p><a href=\"/offers/{}\">Details</a></p><code>{}</code>",
            escape_html(offer.id),
            escape_html(&link)
        ));
        html.push_str("</article>");
    }
    html.push_str("</div></section>");

    html.push_str("<section><h2>Click Reporting</h2><table><thead><tr><th>Offer</th><th>Tracking Identifier</th><th>Clicks</th></tr></thead><tbody>");
    if snapshot.click_summaries.is_empty() {
        html.push_str("<tr><td colspan=\"3\">No clicks recorded in this run.</td></tr>");
    }
    for summary in &snapshot.click_summaries {
        html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
            escape_html(&summary.offer_id),
            escape_html(
                summary
                    .tracking_identifier
                    .as_deref()
                    .unwrap_or("unattributed")
            ),
            summary.clicks
        ));
    }
    html.push_str("</tbody></table></section>");

    html.push_str("<section><h2>Conversion Reporting</h2><table><thead><tr><th>Offer</th><th>Event</th><th>Payout</th><th>Status</th><th>Transaction</th><th>Tracking Identifier</th><th>Postback</th></tr></thead><tbody>");
    if snapshot.conversions.is_empty() {
        html.push_str("<tr><td colspan=\"7\">No conversions recorded in this run.</td></tr>");
    }
    for conversion in &snapshot.conversions {
        html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{} {}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            escape_html(&conversion.offer_id),
            escape_html(&conversion.event_type),
            escape_html(&conversion.payout),
            escape_html(&conversion.currency),
            escape_html(delivery_label(&conversion.delivery.status)),
            escape_html(&conversion.transaction_id),
            escape_html(&conversion.tracking_identifier),
            escape_html(&postback_result(conversion))
        ));
    }
    html.push_str("</tbody></table></section>");
    html.push_str("</main>");
    html.push_str(&page_end());
    html
}

pub fn offer_detail(base_url: &str, offer_id: &str) -> String {
    let mut html = page_start("Fake Offer Details");
    html.push_str("<main>");
    if let Some(offer) = ad_buy_engine_domain::fake_affiliate_offer_by_id(offer_id) {
        html.push_str(&format!(
            "<h1>{}</h1><p>{}</p>",
            escape_html(offer.name),
            escape_html(offer.display_copy)
        ));
        html.push_str(&format!(
            "<p>Tracking link:</p><code>{}</code>",
            escape_html(&fake_affiliate_offer_url(base_url, offer.id))
        ));
    } else {
        html.push_str("<h1>Offer not found</h1>");
    }
    html.push_str("<p><a href=\"/\">Back to dashboard</a></p></main>");
    html.push_str(&page_end());
    html
}

pub fn click_landing(offer_id: &str, outcome: &ClickOutcome) -> String {
    let mut html = page_start("Fake Offer");
    html.push_str("<main>");
    html.push_str(&format!(
        "<h1>Fake offer click recorded</h1><p>Offer: {}</p><p>Tracking identifier: {}</p>",
        escape_html(offer_id),
        escape_html(
            outcome
                .click
                .tracking_identifier
                .as_deref()
                .unwrap_or("unattributed")
        )
    ));
    if let Some(conversion) = &outcome.generated_conversion {
        html.push_str(&format!(
            "<p>Conversion threshold reached: {} {}</p>",
            escape_html(&conversion.event_type),
            escape_html(&conversion.transaction_id)
        ));
    }
    html.push_str("<p><a href=\"/\">Back to dashboard</a></p></main>");
    html.push_str(&page_end());
    html
}

pub fn settings_error(base_url: &str, snapshot: &NetworkSnapshot, error: &str) -> String {
    dashboard(base_url, snapshot, Some(error))
}

fn postback_result(conversion: &crate::state::ConversionRecord) -> String {
    let mut parts = vec![visible_delivery_label(&conversion.delivery.status).to_string()];
    if matches!(conversion.origin, ConversionOrigin::Sample) {
        parts.push("sample".to_string());
    }
    if let Some(status) = conversion.delivery.response_status {
        parts.push(format!("HTTP {status}"));
    }
    if let Some(reason) = conversion.delivery.failure_reason.as_deref() {
        parts.push(reason.to_string());
    }
    parts.join(" / ")
}

fn delivery_label(status: &DeliveryStatus) -> &'static str {
    visible_delivery_label(status)
}

fn page_start(title: &str) -> String {
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\">\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\
         <title>{}</title><style>{}</style></head><body>",
        escape_html(title),
        STYLE
    )
}

fn page_end() -> String {
    "</body></html>".to_string()
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

const STYLE: &str = "
body{font-family:Arial,sans-serif;margin:0;background:#f7f8fb;color:#15171a}
main{max-width:1180px;margin:0 auto;padding:24px}
h1{font-size:32px;margin:0 0 16px}
h2{font-size:22px;margin:28px 0 12px}
h3{font-size:18px;margin:0 0 8px}
section{border-top:1px solid #d9dee8;padding-top:18px}
.offers{display:grid;grid-template-columns:repeat(auto-fit,minmax(260px,1fr));gap:14px}
article{background:#fff;border:1px solid #d9dee8;border-radius:8px;padding:14px}
code{display:block;white-space:normal;word-break:break-all;background:#eef2f7;padding:8px;border-radius:6px}
table{width:100%;border-collapse:collapse;background:#fff}
th,td{text-align:left;border:1px solid #d9dee8;padding:8px;vertical-align:top}
label{display:block;margin:8px 0}
input{width:100%;max-width:760px;padding:8px;border:1px solid #b9c0cc;border-radius:6px}
button{margin:8px 8px 0 0;padding:8px 12px;border:1px solid #1b4d89;border-radius:6px;background:#1f6fba;color:#fff}
.notice{background:#fff4cf;border:1px solid #e5c75f;padding:10px;border-radius:6px}
.inline-actions{margin-top:16px}
";

pub fn event_kind_from_value(value: &str) -> Option<FakeAffiliateOfferKind> {
    match value.trim().to_ascii_lowercase().as_str() {
        "lead" => Some(FakeAffiliateOfferKind::Lead),
        "sale" => Some(FakeAffiliateOfferKind::Sale),
        _ => None,
    }
}
