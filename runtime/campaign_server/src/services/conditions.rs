use std::net::Ipv4Addr;

use ad_buy_engine_domain::{
    ClickContext, ConditionOperator, ConditionRule, ConditionType, TokenValue,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ConditionEvaluation {
    pub matched: bool,
    pub missing_fields: Vec<String>,
}

pub fn evaluate_all(rules: &[ConditionRule], context: &ClickContext) -> ConditionEvaluation {
    let mut missing_fields = Vec::new();
    let mut matched = true;

    for rule in rules.iter().filter(|rule| rule.active) {
        let evaluation = evaluate_rule(rule, context);
        missing_fields.extend(evaluation.missing_fields);
        if !evaluation.matched {
            matched = false;
        }
    }

    ConditionEvaluation {
        matched,
        missing_fields,
    }
}

pub fn query_lookup(query: &[TokenValue], key: &str) -> Option<String> {
    query
        .iter()
        .find(|item| item.key.eq_ignore_ascii_case(key))
        .map(|item| item.value.clone())
}

fn evaluate_rule(rule: &ConditionRule, context: &ClickContext) -> ConditionEvaluation {
    let field_name = condition_field_name(rule);
    let Some(raw_match) = raw_rule_match(rule, context) else {
        return ConditionEvaluation {
            matched: false,
            missing_fields: vec![field_name],
        };
    };

    let matched = match rule.operator {
        ConditionOperator::Include => raw_match,
        ConditionOperator::Exclude => !raw_match,
    };
    ConditionEvaluation {
        matched,
        missing_fields: Vec::new(),
    }
}

fn raw_rule_match(rule: &ConditionRule, context: &ClickContext) -> Option<bool> {
    match rule.condition_type {
        ConditionType::QueryParameter => {
            let key = rule.key.as_deref()?;
            let value = query_lookup(&context.query, key)?;
            Some(value_matches(&value, &rule.values))
        }
        ConditionType::IpRange => {
            let ip_address = context.ip_address.as_deref()?;
            Some(
                rule.values
                    .iter()
                    .any(|range| ip_in_range(ip_address, range)),
            )
        }
        ConditionType::TimeWindow => {
            let minute = context.minute_of_day?;
            let start = rule.start_minute_of_day?;
            let end = rule.end_minute_of_day?;
            Some(minute_in_window(minute, start, end))
        }
        ConditionType::UniqueVisit => context.is_unique_visit,
        ConditionType::Country => scalar_match(context.country.as_deref(), &rule.values),
        ConditionType::Region => scalar_match(context.region.as_deref(), &rule.values),
        ConditionType::City => scalar_match(context.city.as_deref(), &rule.values),
        ConditionType::Isp => scalar_match(context.isp.as_deref(), &rule.values),
        ConditionType::ConnectionType => {
            scalar_match(context.connection_type.as_deref(), &rule.values)
        }
        ConditionType::ProxyType => scalar_match(context.proxy_type.as_deref(), &rule.values),
        ConditionType::Carrier => scalar_match(context.carrier.as_deref(), &rule.values),
        ConditionType::Browser => scalar_match(context.browser.as_deref(), &rule.values),
        ConditionType::OperatingSystem => {
            scalar_match(context.operating_system.as_deref(), &rule.values)
        }
        ConditionType::DeviceType => scalar_match(context.device_type.as_deref(), &rule.values),
        ConditionType::DeviceBrand => scalar_match(context.device_brand.as_deref(), &rule.values),
        ConditionType::Language => scalar_match(context.language.as_deref(), &rule.values),
        ConditionType::Referrer => scalar_match(context.referrer.as_deref(), &rule.values),
        ConditionType::ReferrerDomain => {
            scalar_match(context.referrer_domain.as_deref(), &rule.values)
        }
        ConditionType::Weekday => scalar_match(context.weekday.as_deref(), &rule.values),
    }
}

fn scalar_match(value: Option<&str>, accepted_values: &[String]) -> Option<bool> {
    value.map(|actual| value_matches(actual, accepted_values))
}

fn value_matches(actual: &str, accepted_values: &[String]) -> bool {
    accepted_values
        .iter()
        .any(|expected| actual.eq_ignore_ascii_case(expected.trim()))
}

fn minute_in_window(minute: u16, start: u16, end: u16) -> bool {
    if start <= end {
        minute >= start && minute <= end
    } else {
        minute >= start || minute <= end
    }
}

fn ip_in_range(ip_address: &str, range: &str) -> bool {
    if !range.contains('/') {
        return ip_address == range;
    }

    let mut pieces = range.split('/');
    let Some(network) = pieces.next() else {
        return false;
    };
    let Some(prefix) = pieces.next() else {
        return false;
    };
    if pieces.next().is_some() {
        return false;
    }
    let Ok(network_ip) = network.parse::<Ipv4Addr>() else {
        return false;
    };
    let Ok(ip) = ip_address.parse::<Ipv4Addr>() else {
        return false;
    };
    let Ok(prefix) = prefix.parse::<u32>() else {
        return false;
    };
    if prefix > 32 {
        return false;
    }
    let mask = if prefix == 0 {
        0
    } else {
        u32::MAX << (32 - prefix)
    };
    u32::from(ip) & mask == u32::from(network_ip) & mask
}

fn condition_field_name(rule: &ConditionRule) -> String {
    match rule.condition_type {
        ConditionType::QueryParameter => rule
            .key
            .as_ref()
            .map(|key| format!("query.{key}"))
            .unwrap_or_else(|| "query".to_string()),
        _ => format!("{:?}", rule.condition_type),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ad_buy_engine_domain::{ConditionOperator, ConditionType};

    #[test]
    fn include_exclude_and_missing_data_are_explicit() {
        let context = ClickContext {
            country: Some("US".to_string()),
            ..ClickContext::default()
        };
        let include = ConditionRule {
            id: "1".to_string(),
            condition_type: ConditionType::Country,
            operator: ConditionOperator::Include,
            key: None,
            values: vec!["US".to_string()],
            timezone: None,
            start_minute_of_day: None,
            end_minute_of_day: None,
            active: true,
        };
        let exclude = ConditionRule {
            operator: ConditionOperator::Exclude,
            values: vec!["CA".to_string()],
            ..include.clone()
        };
        let missing = ConditionRule {
            condition_type: ConditionType::City,
            ..include
        };

        assert!(evaluate_all(&[exclude], &context).matched);
        assert!(!evaluate_all(std::slice::from_ref(&missing), &context).matched);
        assert_eq!(
            evaluate_all(&[missing], &context).missing_fields,
            vec!["City".to_string()]
        );
    }
}
