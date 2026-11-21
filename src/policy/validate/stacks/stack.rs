//! Правила для стеков и имён сервисов.

use std::collections::BTreeSet;

use super::service;
use crate::policy::stacks::stack::Stack;
use crate::policy::validate::Issues;

/// Проверяет список стеков.
pub fn check(stacks: &[Stack], has_webhook: bool, issues: &mut Issues) {
    let mut seen = BTreeSet::new();
    for stack in stacks {
        let path = format!("stacks[{}]", stack.name);
        if !is_project_name(&stack.name) {
            issues.push(format!("{path}.name: {NAME_RULE}"));
        }
        if !seen.insert(stack.name.as_str()) {
            issues.push(format!("{path}.name: имя уже используется"));
        }
        check_body(stack, &path, has_webhook, issues);
    }
}

const NAME_RULE: &str =
    "допустимы строчные латинские буквы, цифры, `-` и `_`, первый символ буква или цифра";

fn check_body(stack: &Stack, path: &str, has_webhook: bool, issues: &mut Issues) {
    if stack
        .project
        .as_deref()
        .is_some_and(|project| !is_project_name(project))
    {
        issues.push(format!("{path}.project: {NAME_RULE}"));
    }
    if stack.compose_file.as_os_str().is_empty() {
        issues.push(format!("{path}.compose_file: не может быть пустым"));
    }
    if stack.services.is_empty() {
        issues.push(format!("{path}.services: нужен хотя бы один сервис"));
    }
    for (name, item) in &stack.services {
        let service_path = format!("{path}.services.{name}");
        if !is_service_name(name) {
            issues.push(format!(
                "{service_path}: имя может содержать буквы, цифры, `.`, `_`, `-`"
            ));
        }
        service::check(&service_path, item, has_webhook, issues);
    }
}

fn is_project_name(name: &str) -> bool {
    let mut chars = name.chars();
    let first_ok = chars
        .next()
        .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit());
    first_ok && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
}

fn is_service_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "._-".contains(c))
}
