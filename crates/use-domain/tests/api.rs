use use_domain::{has_subdomain, is_valid_domain, normalize_domain, split_domain_labels};

#[test]
fn detects_valid_domains() {
    assert!(is_valid_domain("example.com"));
}

#[test]
fn rejects_invalid_domains() {
    assert!(!is_valid_domain("bad domain"));
    assert!(!is_valid_domain("localhost"));
}

#[test]
fn splits_labels() {
    assert_eq!(
        split_domain_labels("api.example.com"),
        vec!["api", "example", "com"]
    );
}

#[test]
fn normalizes_domains() {
    assert_eq!(
        normalize_domain("WWW.Example.com."),
        Some("www.example.com".to_string())
    );
}

#[test]
fn detects_subdomains() {
    assert!(has_subdomain("api.example.com"));
    assert!(!has_subdomain("example.com"));
}

#[test]
fn handles_empty_input() {
    assert!(!is_valid_domain(""));
    assert!(normalize_domain("").is_none());
}
