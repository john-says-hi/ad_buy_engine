use ad_buy_engine_domain::{FakeLandingPage, fake_landing_page_catalog, fake_landing_page_url};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContinuationAction {
    pub parameter: String,
    pub label: String,
    pub url: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActionState {
    Ready(Vec<ContinuationAction>),
    Problem(String),
}

pub fn dashboard(base_url: &str) -> String {
    let mut html = page_start("Fake Landing Page Server");
    html.push_str("<main><h1>Fake Landing Page Server</h1>");
    html.push_str(
        "<p>Public-safe preset landers for local Ad Buy Engine continuation testing.</p>",
    );
    html.push_str("<section><h2>Preset Landers</h2><div class=\"landers\">");
    for lander in fake_landing_page_catalog() {
        html.push_str("<article>");
        html.push_str(&format!(
            "<h3>{}</h3><p>{}</p>",
            escape_html(lander.name),
            escape_html(lander.display_copy)
        ));
        html.push_str(&format!(
            "<dl><dt>Preset ID</dt><dd>{}</dd><dt>Role</dt><dd>{:?}</dd>\
             <dt>CTA slots</dt><dd>{}</dd><dt>Vertical</dt><dd>{}</dd></dl>",
            escape_html(lander.id),
            lander.role,
            lander.cta_count,
            escape_html(lander.vertical)
        ));
        html.push_str(&format!(
            "<p>Seed URL template:</p><code>{}</code>",
            escape_html(&fake_landing_page_url(base_url, *lander))
        ));
        html.push_str("</article>");
    }
    html.push_str("</div></section></main>");
    html.push_str(&page_end());
    html
}

pub fn lander_page(lander: FakeLandingPage, action_state: ActionState) -> String {
    let mut html = page_start(lander.name);
    html.push_str("<main>");
    html.push_str(&format!(
        "<p class=\"eyebrow\">Fake landing page preset: {}</p><h1>{}</h1><p>{}</p>",
        escape_html(lander.id),
        escape_html(lander.name),
        escape_html(lander.display_copy)
    ));
    html.push_str(&format!(
        "<dl><dt>Role</dt><dd>{:?}</dd><dt>CTA slots</dt><dd>{}</dd>\
         <dt>Vertical</dt><dd>{}</dd><dt>Tags</dt><dd>{}</dd></dl>",
        lander.role,
        lander.cta_count,
        escape_html(lander.vertical),
        escape_html(&lander.tags.join(", "))
    ));

    match action_state {
        ActionState::Ready(actions) => html.push_str(&page_body_for(lander, &actions)),
        ActionState::Problem(message) => html.push_str(&format!(
            "<section class=\"notice\"><h2>Continuation unavailable</h2><p>{}</p></section>",
            escape_html(&message)
        )),
    }

    html.push_str("<p><a href=\"/\">Back to fake lander catalog</a></p></main>");
    html.push_str(&page_end());
    html
}

pub fn not_found(lander_id: &str) -> String {
    let mut html = page_start("Fake Lander Not Found");
    html.push_str("<main>");
    html.push_str(&format!(
        "<h1>Fake lander not found</h1><p>No fake landing page preset exists for <code>{}</code>.</p>",
        escape_html(lander_id)
    ));
    html.push_str("<p><a href=\"/\">Back to fake lander catalog</a></p></main>");
    html.push_str(&page_end());
    html
}

pub fn local_error(title: &str, message: &str) -> String {
    let mut html = page_start(title);
    html.push_str("<main>");
    html.push_str(&format!(
        "<h1>{}</h1><p>{}</p>",
        escape_html(title),
        escape_html(message)
    ));
    html.push_str("<p><a href=\"/\">Back to fake lander catalog</a></p></main>");
    html.push_str(&page_end());
    html
}

fn page_body_for(lander: FakeLandingPage, actions: &[ContinuationAction]) -> String {
    match lander.id {
        "fake-lander-lead-capture" => lead_capture_body(lander, actions),
        "fake-lander-advertorial" => advertorial_body(actions),
        "fake-lander-after-optin" => after_optin_body(actions),
        "fake-lander-multi-cta" => multi_cta_body(actions),
        _ => standard_body(actions),
    }
}

fn standard_body(actions: &[ContinuationAction]) -> String {
    let Some(action) = actions.first() else {
        return String::new();
    };
    format!(
        "<section><h2>Fake offer preview</h2>\
         <p>This simple page validates one tracked Ad Buy Engine click before continuing.</p>\
         <p><a class=\"cta primary\" data-continuation=\"{}\" href=\"{}\">{}</a></p></section>",
        escape_html(&action.parameter),
        escape_html(&action.url),
        escape_html(&action.label)
    )
}

fn lead_capture_body(lander: FakeLandingPage, actions: &[ContinuationAction]) -> String {
    let Some(action) = actions.first() else {
        return String::new();
    };
    format!(
        "<section><h2>Fake email opt-in</h2>\
         <p>Submit any test value. The server discards it and only continues navigation.</p>\
         <form method=\"post\" action=\"{}/opt-in\">\
         <label>Fake email <input name=\"email\" type=\"email\" placeholder=\"person@example.test\"></label>\
         <input name=\"next\" type=\"hidden\" value=\"{}\">\
         <button class=\"cta primary\" type=\"submit\">{}</button></form></section>",
        escape_html(lander.route_path),
        escape_html(&action.url),
        escape_html(&action.label)
    )
}

fn advertorial_body(actions: &[ContinuationAction]) -> String {
    let Some(action) = actions.first() else {
        return String::new();
    };
    format!(
        "<article class=\"story\"><h2>Fake field report</h2>\
         <p>Our fictional review team compared a sample offer flow, checked the local routing, \
         and confirmed this advertorial contains no real claims or brands.</p>\
         <p>The next step should pass through Ad Buy Engine before reaching the fake offer.</p>\
         <p><a class=\"cta primary\" data-continuation=\"{}\" href=\"{}\">{}</a></p></article>",
        escape_html(&action.parameter),
        escape_html(&action.url),
        escape_html(&action.label)
    )
}

fn after_optin_body(actions: &[ContinuationAction]) -> String {
    let Some(action) = actions.first() else {
        return String::new();
    };
    format!(
        "<section><h2>Fake thank-you step</h2>\
         <p>The fake opt-in step is complete. Continue only if this funnel has another local step.</p>\
         <p><a class=\"cta primary\" data-continuation=\"{}\" href=\"{}\">{}</a></p></section>",
        escape_html(&action.parameter),
        escape_html(&action.url),
        escape_html(&action.label)
    )
}

fn multi_cta_body(actions: &[ContinuationAction]) -> String {
    let mut html = String::from(
        "<section><h2>Fake split-test choices</h2>\
         <p>Each option preserves a separate continuation slot supplied by Ad Buy Engine.</p>\
         <div class=\"cta-grid\">",
    );
    for action in actions {
        html.push_str(&format!(
            "<a class=\"cta\" data-continuation=\"{}\" href=\"{}\">{}</a>",
            escape_html(&action.parameter),
            escape_html(&action.url),
            escape_html(&action.label)
        ));
    }
    html.push_str("</div></section>");
    html
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
main{max-width:1080px;margin:0 auto;padding:24px}
h1{font-size:32px;margin:0 0 12px}
h2{font-size:22px;margin:24px 0 12px}
h3{font-size:18px;margin:0 0 8px}
section,article{border-top:1px solid #d9dee8;padding-top:18px;margin-top:18px}
.landers{display:grid;grid-template-columns:repeat(auto-fit,minmax(260px,1fr));gap:14px}
.landers article{background:#fff;border:1px solid #d9dee8;border-radius:8px;padding:14px;margin:0}
dl{display:grid;grid-template-columns:max-content 1fr;gap:6px 12px}
dt{font-weight:700}
code{display:block;white-space:normal;word-break:break-all;background:#eef2f7;padding:8px;border-radius:6px}
label{display:block;margin:8px 0}
input{width:100%;max-width:520px;padding:8px;border:1px solid #b9c0cc;border-radius:6px}
button,.cta{display:inline-block;margin:8px 8px 0 0;padding:10px 14px;border:1px solid #1b4d89;border-radius:6px;background:#1f6fba;color:#fff;text-decoration:none}
.cta-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:10px}
.notice{background:#fff4cf;border:1px solid #e5c75f;padding:12px;border-radius:6px}
.eyebrow{font-size:13px;text-transform:uppercase;letter-spacing:0;color:#596273}
";
