//! Git-репозиторий с желаемым состоянием.

use serde::Deserialize;

use crate::policy::types::interval::Interval;

/// Репозиторий с compose-файлами и политикой.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Repo {
    /// Адрес репозитория.
    pub url: String,
    /// Отслеживаемая ветка (по умолчанию `main`).
    #[serde(default = "default_branch")]
    pub branch: String,
    /// Как часто проверять новые коммиты (по умолчанию 60 секунд).
    #[serde(default = "default_poll_interval")]
    pub poll_interval: Interval,
}

fn default_branch() -> String {
    "main".to_owned()
}

fn default_poll_interval() -> Interval {
    Interval::from_secs(60)
}
