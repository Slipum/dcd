//! Правила секции webhook.

use crate::policy::sections::webhook::Webhook;
use crate::policy::validate::{Issues, require_text};

/// Проверяет настройки вебхука.
pub fn check(hook: &Webhook, issues: &mut Issues) {
    require_text("webhook.secret_env", &hook.secret_env, issues);
}
