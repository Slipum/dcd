//! Политика одного сервиса Compose.

use serde::Deserialize;

use super::healthcheck::Healthcheck;
use crate::policy::types::interval::Interval;
use crate::policy::types::track::Track;

/// Что запускает проверку обновлений.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Trigger {
    /// Периодический опрос registry.
    Poll,
    /// Вебхук от registry.
    Webhook,
}

/// Нужно ли подтверждение человека.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Approval {
    /// Применять автоматически (по умолчанию).
    #[default]
    Auto,
    /// Ждать подтверждения.
    Manual,
}

/// Стратегия обновления.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Strategy {
    /// Пересоздание контейнера (по умолчанию).
    #[default]
    Recreate,
    /// Зарезервировано; в v1 отклоняется валидацией.
    BlueGreen,
}

/// Поведение при провале деплоя.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rollback {
    /// Откатывать ли при провале healthcheck (по умолчанию да).
    #[serde(default = "default_on_failure")]
    pub on_failure: bool,
}

impl Default for Rollback {
    fn default() -> Self {
        Self {
            on_failure: default_on_failure(),
        }
    }
}

/// Сервис Compose под управлением утилиты.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Service {
    /// Образ без тега и digest.
    pub image: String,
    /// Что отслеживать.
    pub track: Track,
    /// Триггеры (по умолчанию только опрос).
    #[serde(default = "default_triggers")]
    pub triggers: Vec<Trigger>,
    /// Период опроса (по умолчанию 5 минут).
    #[serde(default = "default_poll_interval")]
    pub poll_interval: Interval,
    /// Сколько digest должен оставаться неизменным до обновления (только daemon).
    #[serde(default)]
    pub stable_for: Interval,
    /// Режим подтверждения.
    #[serde(default)]
    pub approval: Approval,
    /// Стратегия обновления.
    #[serde(default)]
    pub strategy: Strategy,
    /// Проверка здоровья.
    #[serde(default)]
    pub healthcheck: Healthcheck,
    /// Откат.
    #[serde(default)]
    pub rollback: Rollback,
}

fn default_on_failure() -> bool {
    true
}

fn default_triggers() -> Vec<Trigger> {
    vec![Trigger::Poll]
}

fn default_poll_interval() -> Interval {
    Interval::from_secs(300)
}
