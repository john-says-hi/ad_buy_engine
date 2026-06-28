use crate::api::FieldError;
use crate::entities::{
    CampaignDraft, DestinationType, FunnelDraft, LandingPageDraft, OfferDraft, OfferSourceDraft,
    TrafficSourceDraft,
};
use crate::funnel::{FunnelPath, FunnelSequence, SequenceType};

pub trait ValidateDraft {
    fn validate(&self) -> Vec<FieldError>;
}

impl ValidateDraft for OfferSourceDraft {
    fn validate(&self) -> Vec<FieldError> {
        required_text_errors(&[("name", &self.name)])
    }
}

impl ValidateDraft for OfferDraft {
    fn validate(&self) -> Vec<FieldError> {
        let mut errors = required_text_errors(&[
            ("offer_source_id", &self.offer_source_id),
            ("name", &self.name),
            ("url", &self.url),
        ]);
        if self.weight == 0 {
            errors.push(field_error("weight", "Weight must be greater than zero"));
        }
        errors
    }
}

impl ValidateDraft for LandingPageDraft {
    fn validate(&self) -> Vec<FieldError> {
        let mut errors = required_text_errors(&[("name", &self.name), ("url", &self.url)]);
        if self.cta_count == 0 {
            errors.push(field_error("cta_count", "CTA count must be at least one"));
        }
        if self.weight == 0 {
            errors.push(field_error("weight", "Weight must be greater than zero"));
        }
        errors
    }
}

impl ValidateDraft for TrafficSourceDraft {
    fn validate(&self) -> Vec<FieldError> {
        required_text_errors(&[("name", &self.name)])
    }
}

impl ValidateDraft for FunnelDraft {
    fn validate(&self) -> Vec<FieldError> {
        let mut errors = required_text_errors(&[("name", &self.name)]);
        if self.default_sequences.is_empty() {
            errors.push(field_error(
                "default_sequences",
                "At least one default sequence is required",
            ));
        }
        errors.extend(validate_sequences(
            "default_sequences",
            &self.default_sequences,
        ));
        errors.extend(validate_sequences(
            "conditional_sequences",
            &self.conditional_sequences,
        ));
        errors
    }
}

impl ValidateDraft for CampaignDraft {
    fn validate(&self) -> Vec<FieldError> {
        let mut errors = required_text_errors(&[
            ("traffic_source_id", &self.traffic_source_id),
            ("name", &self.name),
        ]);
        match self.destination_type {
            DestinationType::Funnel => {
                if self
                    .funnel_id
                    .as_deref()
                    .unwrap_or_default()
                    .trim()
                    .is_empty()
                {
                    errors.push(field_error("funnel_id", "A funnel is required"));
                }
            }
            DestinationType::DirectSequence => {
                if self.direct_sequence.is_none() {
                    errors.push(field_error(
                        "direct_sequence",
                        "A direct sequence is required",
                    ));
                }
            }
        }
        errors
    }
}

fn validate_sequences(prefix: &str, sequences: &[FunnelSequence]) -> Vec<FieldError> {
    let mut errors = Vec::new();
    for (index, sequence) in sequences.iter().enumerate() {
        if sequence.weight == 0 {
            errors.push(field_error(
                format!("{prefix}.{index}.weight"),
                "Sequence weight must be greater than zero",
            ));
        }
        if sequence.active && sequence.paths.is_empty() {
            errors.push(field_error(
                format!("{prefix}.{index}.paths"),
                "Active sequences need at least one path",
            ));
        }
        for (path_index, path) in sequence.paths.iter().enumerate() {
            validate_path(
                &format!("{prefix}.{index}.paths.{path_index}"),
                &sequence.sequence_type,
                path,
                &mut errors,
            );
        }
    }
    errors
}

fn validate_path(
    prefix: &str,
    sequence_type: &SequenceType,
    path: &FunnelPath,
    errors: &mut Vec<FieldError>,
) {
    if path.weight == 0 {
        errors.push(field_error(
            format!("{prefix}.weight"),
            "Path weight must be greater than zero",
        ));
    }
    if matches!(
        sequence_type,
        SequenceType::LandingPageAndOffers | SequenceType::Matrix
    ) && path
        .landing_page_id
        .as_deref()
        .unwrap_or_default()
        .trim()
        .is_empty()
    {
        errors.push(field_error(
            format!("{prefix}.landing_page_id"),
            "A landing page is required for this sequence type",
        ));
    }
    if path.offers.is_empty() && path.children.is_empty() {
        errors.push(field_error(
            format!("{prefix}.offers"),
            "At least one offer or child path is required",
        ));
    }
    for (child_index, child) in path.children.iter().enumerate() {
        validate_path(
            &format!("{prefix}.children.{child_index}"),
            sequence_type,
            child,
            errors,
        );
    }
}

fn required_text_errors(fields: &[(&str, &str)]) -> Vec<FieldError> {
    fields
        .iter()
        .filter_map(|(field, value)| {
            if value.trim().is_empty() {
                Some(field_error(*field, "This field is required"))
            } else {
                None
            }
        })
        .collect()
}

fn field_error(field: impl Into<String>, message: impl Into<String>) -> FieldError {
    FieldError {
        field: field.into(),
        message: message.into(),
    }
}
