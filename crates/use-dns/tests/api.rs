use use_dns::{
    DnsRecordType, format_record_type, is_address_record, is_alias_record, is_mail_record,
    is_text_record, looks_like_dns_name, normalize_dns_name, parse_record_type,
};

#[test]
fn parses_record_types() {
    assert_eq!(parse_record_type("aaaa"), DnsRecordType::AAAA);
}

#[test]
fn formats_record_types() {
    assert_eq!(format_record_type(DnsRecordType::SRV), "SRV");
}

#[test]
fn detects_address_records() {
    assert!(is_address_record(DnsRecordType::A));
    assert!(is_address_record(DnsRecordType::AAAA));
}

#[test]
fn detects_alias_records() {
    assert!(is_alias_record(DnsRecordType::CNAME));
}

#[test]
fn detects_mail_records() {
    assert!(is_mail_record(DnsRecordType::MX));
}

#[test]
fn detects_text_records() {
    assert!(is_text_record(DnsRecordType::TXT));
}

#[test]
fn rejects_malformed_input() {
    assert_eq!(parse_record_type("??"), DnsRecordType::Unknown);
    assert!(!looks_like_dns_name("bad name"));
}

#[test]
fn handles_empty_input() {
    assert_eq!(parse_record_type(""), DnsRecordType::Unknown);
    assert!(normalize_dns_name("").is_none());
}
