use dcd::error::Error;
use dcd::policy::load::from_str;
use dcd::policy::validate::validate;

mod sections;
mod stacks;

const VALID: &str = r#"
version: 1
git:
  url: https://github.com/me/infra.git
registries:
  - name: ghcr
    kind: ghcr
    username_env: U
    token_env: T
stacks:
  - name: blog
    compose_file: stacks/blog/compose.yaml
    services:
      web:
        image: ghcr.io/me/blog
        track: { semver: "^1.2" }
        triggers: [poll, webhook]
        healthcheck:
          source: custom
          test: ["CMD", "true"]
webhook:
  listen: 127.0.0.1:9000
  secret_env: S
"#;

fn issues_of(yaml: &str) -> Vec<String> {
    let policy = from_str(yaml).unwrap();
    let Err(Error::Validation(issues)) = validate(&policy) else {
        panic!("ожидалась ошибка проверки");
    };
    issues
}

#[test]
fn valid_policy_passes() {
    assert!(validate(&from_str(VALID).unwrap()).is_ok());
}

#[test]
fn minimal_policy_passes() {
    assert!(validate(&from_str("version: 1\n").unwrap()).is_ok());
}

#[test]
fn all_issues_are_collected() {
    let yaml = VALID
        .replace("kind: ghcr", "kind: gitea")
        .replace("image: ghcr.io/me/blog", "image: app:1")
        .replace("secret_env: S", "secret_env: \"\"");
    assert_eq!(issues_of(&yaml).len(), 3);
}

#[test]
fn webhook_trigger_without_section_is_reported() {
    let yaml = VALID.replace("webhook:\n  listen: 127.0.0.1:9000\n  secret_env: S\n", "");
    assert!(issues_of(&yaml)[0].contains("секцию webhook"));
}

#[test]
fn validation_error_lists_every_issue() {
    let error = Error::Validation(vec!["a: x".to_owned(), "b: y".to_owned()]);
    let text = error.to_string();
    assert!(text.contains("не прошла проверку") && text.contains("a: x") && text.contains("b: y"));
}
