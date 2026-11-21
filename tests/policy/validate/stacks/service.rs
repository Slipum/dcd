use dcd::policy::load::from_str;
use dcd::policy::stacks::service::Service;
use dcd::policy::validate::stacks::service::check;

fn service_with(image: &str, extra: &str) -> Service {
    let yaml = format!(
        "version: 1\nstacks:\n  - name: s\n    compose_file: c.yaml\n    services:\n      web:\n        image: {image}\n        track: {{ tag: latest }}\n{extra}"
    );
    from_str(&yaml)
        .unwrap()
        .stacks
        .remove(0)
        .services
        .remove("web")
        .unwrap()
}

fn run(service: &Service, has_webhook: bool) -> Vec<String> {
    let mut issues = Vec::new();
    check("p", service, has_webhook, &mut issues);
    issues
}

fn run_with(extra: &str, has_webhook: bool) -> Vec<String> {
    run(&service_with("ghcr.io/me/blog", extra), has_webhook)
}

#[test]
fn default_service_is_valid() {
    assert!(run_with("", false).is_empty());
}

#[test]
fn blue_green_is_rejected_in_v1() {
    assert!(run_with("        strategy: blue_green\n", false)[0].contains("не поддерживается"));
}

#[test]
fn empty_triggers_are_rejected() {
    assert!(run_with("        triggers: []\n", false)[0].contains("хотя бы один"));
}

#[test]
fn repeated_triggers_are_rejected() {
    assert!(run_with("        triggers: [poll, poll]\n", false)[0].contains("повторяться"));
}

#[test]
fn webhook_trigger_requires_webhook_section() {
    assert!(run_with("        triggers: [webhook]\n", false)[0].contains("секцию webhook"));
}

#[test]
fn webhook_trigger_with_section_is_valid() {
    assert!(run_with("        triggers: [webhook]\n", true).is_empty());
}

#[test]
fn zero_poll_interval_is_rejected_for_poll_trigger() {
    assert!(run_with("        poll_interval: 0s\n", false)[0].contains("poll_interval"));
}

#[test]
fn zero_poll_interval_is_fine_for_webhook_only() {
    assert!(
        run_with(
            "        triggers: [webhook]\n        poll_interval: 0s\n",
            true
        )
        .is_empty()
    );
}

#[test]
fn nested_issues_carry_the_full_path() {
    let service = service_with("app:1.2", "");
    assert!(run(&service, false)[0].starts_with("p.image"));
}
