//! Правила секции git.

use crate::policy::sections::git::Repo;
use crate::policy::validate::{Issues, require_text};

/// Проверяет git-источник.
pub fn check(repo: &Repo, issues: &mut Issues) {
    require_text("git.url", &repo.url, issues);
    require_text("git.branch", &repo.branch, issues);
    if repo.poll_interval.0.is_zero() {
        issues.push("git.poll_interval: должен быть больше нуля".to_owned());
    }
}
