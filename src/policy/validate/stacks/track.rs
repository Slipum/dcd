//! Правила для отслеживаемой версии.

use crate::policy::types::track::Track;
use crate::policy::validate::{Issues, require_text};

/// Проверяет значение `track`; корректность диапазона semver проверяется позже, в клиенте registry.
pub fn check(path: &str, track: &Track, issues: &mut Issues) {
    match track {
        Track::Tag(tag) => require_text(&format!("{path}.tag"), tag, issues),
        Track::Semver(range) => require_text(&format!("{path}.semver"), range, issues),
        Track::Digest(digest) => {
            if !is_sha256(digest) {
                issues.push(format!(
                    "{path}.digest: ожидается sha256:<64 hex-символа в нижнем регистре>"
                ));
            }
        }
    }
}

fn is_sha256(value: &str) -> bool {
    value.strip_prefix("sha256:").is_some_and(|hex| {
        hex.len() == 64
            && hex
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}
