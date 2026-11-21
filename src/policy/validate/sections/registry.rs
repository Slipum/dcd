//! Правила секции registries.

use std::collections::BTreeSet;

use crate::policy::sections::registry::{Kind, Registry};
use crate::policy::validate::{Issues, require_text};

/// Проверяет список registry.
pub fn check(registries: &[Registry], issues: &mut Issues) {
    let mut seen = BTreeSet::new();
    for (index, registry) in registries.iter().enumerate() {
        let path = format!("registries[{index}]");
        require_text(&format!("{path}.name"), &registry.name, issues);
        if !seen.insert(registry.name.as_str()) {
            issues.push(format!(
                "{path}.name: имя `{}` уже используется",
                registry.name
            ));
        }
        check_host(registry, &path, issues);
        check_credentials(registry, &path, issues);
    }
}

fn check_host(registry: &Registry, path: &str, issues: &mut Issues) {
    let needs_host = matches!(registry.kind, Kind::Gitea | Kind::Forgejo);
    let has_host = registry
        .host
        .as_deref()
        .is_some_and(|host| !host.trim().is_empty());
    if needs_host && !has_host {
        issues.push(format!("{path}.host: обязателен для gitea и forgejo"));
    }
}

fn check_credentials(registry: &Registry, path: &str, issues: &mut Issues) {
    if registry.username_env.is_some() != registry.token_env.is_some() {
        issues.push(format!("{path}: username_env и token_env задаются вместе"));
    }
}
