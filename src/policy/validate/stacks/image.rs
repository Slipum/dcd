//! Правила для имени образа: без тега и digest.

use crate::policy::validate::Issues;

/// Проверяет ссылку на образ; версия задаётся отдельно в `track`.
pub fn check(path: &str, image: &str, issues: &mut Issues) {
    if image.trim().is_empty() {
        issues.push(format!("{path}: не может быть пустым"));
        return;
    }
    if image.chars().any(char::is_whitespace) {
        issues.push(format!("{path}: не может содержать пробелы"));
    }
    let last_segment = image.rsplit('/').next().unwrap_or(image);
    if image.contains('@') {
        issues.push(format!(
            "{path}: укажите образ без digest (используйте track.digest)"
        ));
    } else if last_segment.contains(':') {
        issues.push(format!(
            "{path}: укажите образ без тега (версия задаётся в track)"
        ));
    }
}
