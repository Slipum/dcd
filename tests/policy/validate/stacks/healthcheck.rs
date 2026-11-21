use dcd::policy::stacks::healthcheck::{Healthcheck, Source};
use dcd::policy::types::interval::Interval;
use dcd::policy::validate::stacks::healthcheck::check;

fn run(health: &Healthcheck) -> Vec<String> {
    let mut issues = Vec::new();
    check("p", health, &mut issues);
    issues
}

fn custom(test: Option<Vec<String>>) -> Healthcheck {
    Healthcheck {
        source: Source::Custom,
        test,
        ..Healthcheck::default()
    }
}

#[test]
fn default_healthcheck_is_valid() {
    assert!(run(&Healthcheck::default()).is_empty());
}

#[test]
fn zero_timeout_is_rejected() {
    let health = Healthcheck {
        timeout: Interval::from_secs(0),
        ..Healthcheck::default()
    };
    assert!(run(&health)[0].contains("timeout"));
}

#[test]
fn custom_with_test_is_valid() {
    assert!(run(&custom(Some(vec!["CMD".to_owned()]))).is_empty());
}

#[test]
fn custom_without_test_is_rejected() {
    assert!(run(&custom(None))[0].contains("обязателен"));
}

#[test]
fn custom_with_empty_test_is_rejected() {
    assert!(run(&custom(Some(Vec::new())))[0].contains("обязателен"));
}

#[test]
fn custom_with_zero_retries_is_rejected() {
    let health = Healthcheck {
        retries: Some(0),
        ..custom(Some(vec!["CMD".to_owned()]))
    };
    assert!(run(&health)[0].contains("retries"));
}

#[test]
fn image_source_with_test_is_rejected() {
    let health = Healthcheck {
        test: Some(vec!["CMD".to_owned()]),
        ..Healthcheck::default()
    };
    assert!(run(&health)[0].contains("только для source: custom"));
}

#[test]
fn disabled_source_with_interval_is_rejected() {
    let health = Healthcheck {
        source: Source::Disabled,
        interval: Some(Interval::from_secs(5)),
        ..Healthcheck::default()
    };
    assert!(run(&health)[0].contains("только для source: custom"));
}

#[test]
fn disabled_source_alone_is_valid() {
    let health = Healthcheck {
        source: Source::Disabled,
        ..Healthcheck::default()
    };
    assert!(run(&health).is_empty());
}
