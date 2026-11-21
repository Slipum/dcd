//! Единый тип ошибки приложения.

use std::path::PathBuf;

/// Ошибка приложения.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Файл не удалось прочитать.
    #[error("не удалось прочитать {path}: {source}")]
    Io {
        /// Путь к файлу.
        path: PathBuf,
        /// Исходная ошибка ввода-вывода.
        source: std::io::Error,
    },
    /// Политика не разобрана или не соответствует схеме.
    #[error("ошибка политики: {0}")]
    Policy(String),
    /// Политика разобрана, но нарушает правила согласованности.
    #[error("политика не прошла проверку:\n{}", .0.join("\n"))]
    Validation(Vec<String>),
}

/// Результат операций приложения.
pub type Result<T> = std::result::Result<T, Error>;
