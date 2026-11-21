//! Проверка здоровья сервиса после деплоя.

use serde::Deserialize;

use crate::policy::types::interval::Interval;

/// Откуда берётся healthcheck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    /// Из образа (по умолчанию).
    #[default]
    Image,
    /// Из политики; приоритетнее образа.
    Custom,
    /// Отключён: откат и blue-green недоступны.
    #[serde(rename = "none")]
    Disabled,
}

/// Параметры healthcheck.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Healthcheck {
    /// Источник проверки.
    #[serde(default)]
    pub source: Source,
    /// Сколько ждать успешной проверки (по умолчанию 60 секунд).
    #[serde(default = "default_timeout")]
    pub timeout: Interval,
    /// Команда проверки (формат как в Compose), только для `custom`.
    pub test: Option<Vec<String>>,
    /// Период между проверками.
    pub interval: Option<Interval>,
    /// Число неудач до признания сервиса нездоровым.
    pub retries: Option<u32>,
}

impl Default for Healthcheck {
    fn default() -> Self {
        Self {
            source: Source::default(),
            timeout: default_timeout(),
            test: None,
            interval: None,
            retries: None,
        }
    }
}

fn default_timeout() -> Interval {
    Interval::from_secs(60)
}
