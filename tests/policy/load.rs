use std::path::Path;

use dcd::error::Error;
use dcd::policy::load::{from_file, from_str};

const SERVICE: &str = r"
version: 1
stacks:
  - name: blog
    compose_file: c.yaml
    services:
      web:
        image: ghcr.io/me/blog
        track: { tag: latest }
";

fn error_of(yaml: &str) -> String {
    from_str(yaml).unwrap_err().to_string()
}

#[test]
fn accepts_supported_version() {
    assert_eq!(from_str("version: 1\n").unwrap().version, 1);
}

#[test]
fn rejects_missing_version() {
    assert!(error_of("stacks: []\n").contains("version"));
}

#[test]
fn rejects_unsupported_version() {
    assert!(error_of("version: 2\n").contains("неподдерживаемая версия"));
}

#[test]
fn rejects_non_numeric_version() {
    assert!(!error_of("version: x\n").is_empty());
}

#[test]
fn rejects_empty_document() {
    assert!(!error_of("").is_empty());
}

#[test]
fn rejects_malformed_yaml() {
    assert!(!error_of("version: [1\n").is_empty());
}

#[test]
fn rejects_unknown_top_level_field() {
    assert!(error_of("version: 1\nfoo: 1\n").contains("foo"));
}

#[test]
fn rejects_unknown_service_field() {
    let yaml = SERVICE.replace(
        "image: ghcr.io/me/blog",
        "image: ghcr.io/me/blog\n        colour: red",
    );
    assert!(error_of(&yaml).contains("colour"));
}

#[test]
fn rejects_missing_track() {
    let yaml = SERVICE.replace("        track: { tag: latest }\n", "");
    assert!(error_of(&yaml).contains("track"));
}

#[test]
fn rejects_unknown_registry_kind() {
    let yaml = "version: 1\nregistries:\n  - name: r\n    kind: bitbucket\n";
    assert!(error_of(yaml).contains("bitbucket"));
}

#[test]
fn rejects_invalid_interval() {
    let yaml = SERVICE.replace(
        "track: { tag: latest }",
        "track: { tag: latest }\n        poll_interval: 5d",
    );
    assert!(error_of(&yaml).contains("5d"));
}

#[test]
fn rejects_wrong_section_type() {
    assert!(!error_of("version: 1\nstacks: text\n").is_empty());
}

#[test]
fn rejects_invalid_webhook_address() {
    let yaml = "version: 1\nwebhook:\n  listen: not-an-address\n  secret_env: S\n";
    assert!(!error_of(yaml).is_empty());
}

#[test]
fn from_file_reports_missing_file() {
    let result = from_file(Path::new("/definitely/not/here/policy.yaml"));
    assert!(matches!(result, Err(Error::Io { .. })));
}

#[test]
fn from_file_reads_valid_policy() {
    let path = std::env::temp_dir().join(format!("dcd-policy-{}.yaml", std::process::id()));
    std::fs::write(&path, "version: 1\n").unwrap();
    let result = from_file(&path);
    std::fs::remove_file(&path).unwrap();
    assert_eq!(result.unwrap().version, 1);
}
