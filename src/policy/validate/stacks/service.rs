//! Правила для одного сервиса.

use super::{healthcheck, image, track};
use crate::policy::stacks::service::{Service, Strategy, Trigger};
use crate::policy::validate::Issues;

/// Проверяет сервис; `has_webhook` — есть ли в политике секция webhook.
pub fn check(path: &str, service: &Service, has_webhook: bool, issues: &mut Issues) {
    image::check(&format!("{path}.image"), &service.image, issues);
    track::check(&format!("{path}.track"), &service.track, issues);
    healthcheck::check(&format!("{path}.healthcheck"), &service.healthcheck, issues);
    check_triggers(path, service, has_webhook, issues);
    if service.strategy == Strategy::BlueGreen {
        issues.push(format!(
            "{path}.strategy: blue_green не поддерживается в v1"
        ));
    }
}

fn check_triggers(path: &str, service: &Service, has_webhook: bool, issues: &mut Issues) {
    let triggers = &service.triggers;
    if triggers.is_empty() {
        issues.push(format!("{path}.triggers: нужен хотя бы один"));
        return;
    }
    if has_duplicates(triggers) {
        issues.push(format!("{path}.triggers: значения не должны повторяться"));
    }
    if triggers.contains(&Trigger::Webhook) && !has_webhook {
        issues.push(format!("{path}.triggers: webhook требует секцию webhook"));
    }
    if triggers.contains(&Trigger::Poll) && service.poll_interval.0.is_zero() {
        issues.push(format!("{path}.poll_interval: должен быть больше нуля"));
    }
}

fn has_duplicates(triggers: &[Trigger]) -> bool {
    triggers
        .iter()
        .enumerate()
        .any(|(index, trigger)| triggers[..index].contains(trigger))
}
