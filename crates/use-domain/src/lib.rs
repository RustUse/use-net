#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// Stores a normalized domain name and its labels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Domain {
    /// Normalized domain value.
    pub value: String,
    /// Individual domain labels.
    pub labels: Vec<String>,
}

fn is_valid_label(label: &str) -> bool {
    !label.is_empty()
        && label.len() <= 63
        && !label.starts_with('-')
        && !label.ends_with('-')
        && label
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '-')
}

fn normalize_candidate(input: &str) -> Option<String> {
    let trimmed = input.trim().trim_end_matches('.');

    if trimmed.is_empty()
        || trimmed.len() > 253
        || trimmed.contains(':')
        || trimmed.contains('/')
        || trimmed.chars().any(char::is_whitespace)
    {
        return None;
    }

    let normalized = trimmed.to_ascii_lowercase();

    if normalized.split('.').all(is_valid_label) {
        Some(normalized)
    } else {
        None
    }
}

/// Returns `true` when the input is a valid ASCII domain name with at least one dot.
pub fn is_valid_domain(input: &str) -> bool {
    normalize_candidate(input).is_some_and(|domain| domain.contains('.'))
}

/// Returns `true` when the input is a valid ASCII hostname or domain name.
pub fn is_valid_hostname(input: &str) -> bool {
    normalize_candidate(input).is_some()
}

/// Normalizes a valid domain name.
pub fn normalize_domain(input: &str) -> Option<String> {
    normalize_candidate(input).filter(|domain| domain.contains('.'))
}

/// Splits a valid domain name into labels.
pub fn split_domain_labels(input: &str) -> Vec<String> {
    normalize_domain(input)
        .map(|domain| domain.split('.').map(String::from).collect())
        .unwrap_or_default()
}

/// Returns the number of labels in a valid domain name.
pub fn domain_depth(input: &str) -> usize {
    split_domain_labels(input).len()
}

/// Returns a naive root-domain guess based on the last two labels.
pub fn root_domain_guess(input: &str) -> Option<String> {
    let labels = split_domain_labels(input);

    if labels.len() >= 2 {
        Some(labels[labels.len() - 2..].join("."))
    } else {
        None
    }
}

/// Returns `true` when the domain has more than two labels.
pub fn has_subdomain(input: &str) -> bool {
    domain_depth(input) > 2
}

/// Returns `true` when the normalized domain is ASCII.
pub fn is_ascii_domain(input: &str) -> bool {
    normalize_domain(input).is_some_and(|domain| domain.is_ascii())
}
