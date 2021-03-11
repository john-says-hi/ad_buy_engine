table! {
    account_table (account_id) {
        account_id -> Varchar,
        account_data -> Varchar,
        last_updated -> Int8,
    }
}

table! {
    campaign_table (campaign_id) {
        campaign_id -> Varchar,
        account_id -> Varchar,
        campaign_data -> Varchar,
        last_updated -> Int8,
    }
}

table! {
    funnel_table (funnel_id) {
        funnel_id -> Varchar,
        account_id -> Varchar,
        funnel_data -> Varchar,
        last_updated -> Int8,
    }
}

table! {
    invitation_table (invitation_id) {
        invitation_id -> Varchar,
        email -> Varchar,
        email_confirmed -> Bool,
        expires_at -> Timestamp,
    }
}

table! {
    landing_page_table (landing_page_id) {
        landing_page_id -> Varchar,
        account_id -> Varchar,
        landing_page_data -> Varchar,
        last_updated -> Int8,
    }
}

table! {
    offer_source_table (offer_source_id) {
        offer_source_id -> Varchar,
        account_id -> Varchar,
        offer_source_data -> Varchar,
        last_updated -> Int8,
    }
}

table! {
    offer_table (offer_id) {
        offer_id -> Varchar,
        account_id -> Varchar,
        offer_data -> Varchar,
        last_updated -> Int8,
    }
}

table! {
    traffic_source_table (traffic_source_id) {
        traffic_source_id -> Varchar,
        account_id -> Varchar,
        traffic_source_data -> Varchar,
        last_updated -> Int8,
    }
}

table! {
    user_table (user_id) {
        user_id -> Varchar,
        account_id -> Varchar,
        email -> Varchar,
        password -> Varchar,
        last_updated -> Int8,
    }
}

table! {
    visit_ledger_table (account_id) {
        account_id -> Varchar,
        visit_ids -> Varchar,
    }
}

table! {
    visit_table (click_id) {
        click_id -> Varchar,
        account_id -> Varchar,
        visit_data -> Varchar,
        last_updated -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    account_table,
    campaign_table,
    funnel_table,
    invitation_table,
    landing_page_table,
    offer_source_table,
    offer_table,
    traffic_source_table,
    user_table,
    visit_ledger_table,
    visit_table,
);
