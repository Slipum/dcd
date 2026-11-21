//! Опциональное хранилище истории деплоев.

use std::path::PathBuf;

use serde::Deserialize;

/// Настройки файла истории (журнал, не источник истины).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    /// Путь к файлу истории; без него история не ведётся.
    pub history_file: Option<PathBuf>,
}
