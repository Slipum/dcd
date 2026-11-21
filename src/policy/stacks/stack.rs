//! Compose-стек и политика ресурсов.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::Deserialize;

use super::service::Service;

/// Что делать с ресурсом при обновлении.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResourceMode {
    /// Оставить без изменений (по умолчанию).
    #[default]
    Keep,
    /// Пересоздать.
    Recreate,
}

/// Политика для volumes и сетей; external-сети не затрагиваются никогда.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Resources {
    /// Volumes.
    #[serde(default)]
    pub volumes: ResourceMode,
    /// Сети.
    #[serde(default)]
    pub networks: ResourceMode,
}

/// Compose-стек.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stack {
    /// Имя стека.
    pub name: String,
    /// Имя Compose-проекта; по умолчанию берётся имя стека.
    pub project: Option<String>,
    /// Путь к compose-файлу.
    pub compose_file: PathBuf,
    /// Политика ресурсов.
    #[serde(default)]
    pub resources: Resources,
    /// Сервисы под управлением; порядок фиксирован (по имени).
    #[serde(default)]
    pub services: BTreeMap<String, Service>,
}
