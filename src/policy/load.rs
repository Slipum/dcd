//! Загрузка политики из строки или файла.

use std::path::Path;

use serde::Deserialize;

use super::document::Policy;
use crate::error::{Error, Result};

/// Поддерживаемая версия схемы.
pub const SUPPORTED_VERSION: u32 = 1;

#[derive(Deserialize)]
struct Header {
    version: u32,
}

/// Разбирает политику из YAML-строки.
///
/// # Errors
/// Ошибка, если YAML некорректен, версия не поддерживается или документ не соответствует схеме.
pub fn from_str(text: &str) -> Result<Policy> {
    check_version(text)?;
    serde_norway::from_str(text).map_err(|error| Error::Policy(error.to_string()))
}

/// Читает и разбирает политику из файла.
///
/// # Errors
/// Ошибка чтения файла или любая ошибка из `from_str`.
pub fn from_file(path: &Path) -> Result<Policy> {
    let text = std::fs::read_to_string(path).map_err(|source| Error::Io {
        path: path.to_path_buf(),
        source,
    })?;
    from_str(&text)
}

/// Версия проверяется до полного разбора: схема других версий может отличаться.
fn check_version(text: &str) -> Result<()> {
    let header: Header =
        serde_norway::from_str(text).map_err(|error| Error::Policy(error.to_string()))?;
    if header.version == SUPPORTED_VERSION {
        return Ok(());
    }
    Err(Error::Policy(format!(
        "неподдерживаемая версия политики: {} (поддерживается {SUPPORTED_VERSION})",
        header.version
    )))
}
