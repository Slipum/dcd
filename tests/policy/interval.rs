use dcd::policy::types::interval::Interval;

fn secs(text: &str) -> u64 {
    Interval::parse(text).unwrap().0.as_secs()
}

fn error_of(text: &str) -> String {
    Interval::parse(text).unwrap_err()
}

#[test]
fn parses_seconds() {
    assert_eq!(secs("30s"), 30);
}

#[test]
fn parses_minutes() {
    assert_eq!(secs("5m"), 300);
}

#[test]
fn parses_hours() {
    assert_eq!(secs("2h"), 7200);
}

#[test]
fn parses_zero() {
    assert_eq!(secs("0s"), 0);
}

#[test]
fn rejects_empty() {
    assert!(error_of("").contains("пустой"));
}

#[test]
fn rejects_missing_unit() {
    assert!(error_of("30").contains("единиц"));
}

#[test]
fn rejects_unknown_unit() {
    assert!(error_of("5d").contains("единиц"));
}

#[test]
fn rejects_missing_number() {
    assert!(error_of("s").contains("целое"));
}

#[test]
fn rejects_negative_number() {
    assert!(error_of("-5s").contains("целое"));
}

#[test]
fn rejects_fraction() {
    assert!(error_of("1.5s").contains("целое"));
}

#[test]
fn rejects_plus_sign() {
    assert!(error_of("+5s").contains("целое"));
}

#[test]
fn rejects_space_before_unit() {
    assert!(error_of("5 m").contains("целое"));
}

#[test]
fn rejects_number_beyond_u64() {
    assert!(error_of("99999999999999999999s").contains("большое"));
}

#[test]
fn rejects_overflow_after_multiplication() {
    assert!(error_of("18446744073709551615h").contains("большое"));
}

#[test]
fn deserializes_from_yaml_string() {
    let value: Interval = serde_norway::from_str("30s").unwrap();
    assert_eq!(value, Interval::from_secs(30));
}

#[test]
fn deserialization_reports_invalid_value() {
    let result: Result<Interval, _> = serde_norway::from_str("30x");
    assert!(result.is_err());
}
