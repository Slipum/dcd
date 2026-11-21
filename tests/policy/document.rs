use std::path::PathBuf;

use dcd::policy::load::from_str;
use dcd::policy::sections::registry::Kind;
use dcd::policy::stacks::healthcheck::Source;
use dcd::policy::stacks::service::{Approval, Strategy, Trigger};
use dcd::policy::stacks::stack::ResourceMode;
use dcd::policy::types::interval::Interval;
use dcd::policy::types::track::Track;

const SERVICE_ONLY: &str = r"
version: 1
stacks:
  - name: blog
    compose_file: stacks/blog/compose.yaml
    services:
      web:
        image: ghcr.io/me/blog
        track: { tag: latest }
";

const FULL: &str = r#"
version: 1
git:
  url: https://github.com/me/infra.git
  branch: stable
  poll_interval: 2m
registries:
  - name: ghcr
    kind: ghcr
    host: ghcr.io
    username_env: GHCR_USER
    token_env: GHCR_TOKEN
stacks:
  - name: blog
    project: blogproj
    compose_file: stacks/blog/compose.yaml
    resources: { volumes: recreate, networks: keep }
    services:
      web:
        image: ghcr.io/me/blog
        track: { semver: "^1.2" }
        triggers: [poll, webhook]
        poll_interval: 10m
        stable_for: 30s
        approval: manual
        strategy: blue_green
        healthcheck:
          source: custom
          timeout: 90s
          test: ["CMD", "curl", "-f", "http://localhost/"]
          interval: 10s
          retries: 3
        rollback: { on_failure: false }
webhook:
  listen: 127.0.0.1:9000
  secret_env: DCD_WEBHOOK_SECRET
state:
  history_file: /var/lib/dcd/history.jsonl
"#;

#[test]
fn minimal_policy_has_empty_sections() {
    let policy = from_str("version: 1\n").unwrap();
    assert!(policy.stacks.is_empty());
    assert!(policy.registries.is_empty());
    assert!(policy.git.is_none() && policy.webhook.is_none() && policy.state.is_none());
}

#[test]
fn service_defaults_are_applied() {
    let policy = from_str(SERVICE_ONLY).unwrap();
    let service = &policy.stacks[0].services["web"];
    assert_eq!(service.track, Track::Tag("latest".to_owned()));
    assert_eq!(service.triggers, vec![Trigger::Poll]);
    assert_eq!(service.poll_interval, Interval::from_secs(300));
    assert_eq!(service.stable_for, Interval::from_secs(0));
    assert_eq!(service.approval, Approval::Auto);
    assert_eq!(service.strategy, Strategy::Recreate);
    assert_eq!(service.healthcheck.source, Source::Image);
    assert_eq!(service.healthcheck.timeout, Interval::from_secs(60));
    assert!(service.rollback.on_failure);
}

#[test]
fn stack_defaults_are_applied() {
    let stack = &from_str(SERVICE_ONLY).unwrap().stacks[0];
    assert_eq!(stack.project, None);
    assert_eq!(stack.resources.volumes, ResourceMode::Keep);
    assert_eq!(stack.resources.networks, ResourceMode::Keep);
}

#[test]
fn full_policy_keeps_every_value() {
    let policy = from_str(FULL).unwrap();
    let stack = &policy.stacks[0];
    let service = &stack.services["web"];
    assert_eq!(policy.git.as_ref().unwrap().branch, "stable");
    assert_eq!(policy.registries[0].kind, Kind::Ghcr);
    assert_eq!(stack.project.as_deref(), Some("blogproj"));
    assert_eq!(stack.resources.volumes, ResourceMode::Recreate);
    assert_eq!(service.track, Track::Semver("^1.2".to_owned()));
    assert_eq!(service.triggers, vec![Trigger::Poll, Trigger::Webhook]);
    assert_eq!(service.approval, Approval::Manual);
    assert_eq!(service.strategy, Strategy::BlueGreen);
    assert!(!service.rollback.on_failure);
}

#[test]
fn full_policy_keeps_healthcheck_webhook_and_state() {
    let policy = from_str(FULL).unwrap();
    let check = &policy.stacks[0].services["web"].healthcheck;
    assert_eq!(check.source, Source::Custom);
    assert_eq!(check.timeout, Interval::from_secs(90));
    assert_eq!(check.retries, Some(3));
    assert_eq!(policy.webhook.unwrap().listen.port(), 9000);
    let history = policy.state.unwrap().history_file;
    assert_eq!(history, Some(PathBuf::from("/var/lib/dcd/history.jsonl")));
}

#[test]
fn every_registry_kind_is_recognised() {
    for (name, kind) in [
        ("dockerhub", Kind::Dockerhub),
        ("ghcr", Kind::Ghcr),
        ("gitea", Kind::Gitea),
        ("forgejo", Kind::Forgejo),
    ] {
        let yaml = format!("version: 1\nregistries:\n  - name: r\n    kind: {name}\n");
        assert_eq!(from_str(&yaml).unwrap().registries[0].kind, kind);
    }
}

#[test]
fn healthcheck_none_is_recognised() {
    let yaml = SERVICE_ONLY.replace(
        "track: { tag: latest }",
        "track: { tag: latest }\n        healthcheck: { source: none }",
    );
    let policy = from_str(&yaml).unwrap();
    assert_eq!(
        policy.stacks[0].services["web"].healthcheck.source,
        Source::Disabled
    );
}

#[test]
fn empty_rollback_mapping_keeps_default() {
    let yaml = SERVICE_ONLY.replace(
        "track: { tag: latest }",
        "track: { tag: latest }\n        rollback: {}",
    );
    let policy = from_str(&yaml).unwrap();
    assert!(policy.stacks[0].services["web"].rollback.on_failure);
}
