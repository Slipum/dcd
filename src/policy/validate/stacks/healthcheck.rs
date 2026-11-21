//! Правила для healthcheck.

use crate::policy::stacks::healthcheck::{Healthcheck, Source};
use crate::policy::validate::Issues;

/// Проверяет настройки healthcheck.
pub fn check(path: &str, health: &Healthcheck, issues: &mut Issues) {
    if health.timeout.0.is_zero() {
        issues.push(format!("{path}.timeout: должен быть больше нуля"));
    }
    if health.source == Source::Custom {
        check_custom(path, health, issues);
    } else if health.test.is_some() || health.interval.is_some() || health.retries.is_some() {
        issues.push(format!(
            "{path}: test, interval и retries допустимы только для source: custom"
        ));
    }
}

fn check_custom(path: &str, health: &Healthcheck, issues: &mut Issues) {
    if health.test.as_ref().is_none_or(Vec::is_empty) {
        issues.push(format!("{path}.test: обязателен для source: custom"));
    }
    if health.retries == Some(0) {
        issues.push(format!("{path}.retries: должно быть не меньше 1"));
    }
}
