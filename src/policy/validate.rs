//! Проверка согласованности политики: собирает все нарушения, а не только первое.

pub mod sections;
pub mod stacks;

use super::document::Policy;
use crate::error::{Error, Result};

/// Список найденных нарушений в виде `путь: описание`.
pub type Issues = Vec<String>;

/// Проверяет политику целиком.
///
/// # Errors
/// `Error::Validation` со списком всех нарушений.
pub fn validate(policy: &Policy) -> Result<()> {
    let mut issues = Issues::new();
    if let Some(repo) = &policy.git {
        sections::git::check(repo, &mut issues);
    }
    sections::registry::check(&policy.registries, &mut issues);
    stacks::stack::check(&policy.stacks, policy.webhook.is_some(), &mut issues);
    if let Some(hook) = &policy.webhook {
        sections::webhook::check(hook, &mut issues);
    }
    if issues.is_empty() {
        Ok(())
    } else {
        Err(Error::Validation(issues))
    }
}

/// Общее правило: текстовое поле не должно быть пустым.
pub(crate) fn require_text(path: &str, value: &str, issues: &mut Issues) {
    if value.trim().is_empty() {
        issues.push(format!("{path}: не может быть пустым"));
    }
}
