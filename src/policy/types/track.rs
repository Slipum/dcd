//! Правило отслеживания версии образа: ровно одно из `tag`, `semver`, `digest`.

use serde::Deserialize;

/// Сырая форма из YAML; в остальном коде используется только `Track`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawTrack {
    tag: Option<String>,
    semver: Option<String>,
    digest: Option<String>,
}

/// Что именно отслеживается у образа.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(try_from = "RawTrack")]
pub enum Track {
    /// Конкретный тег, например `latest`.
    Tag(String),
    /// Диапазон semver, например `^1.2`.
    Semver(String),
    /// Фиксированный digest.
    Digest(String),
}

impl TryFrom<RawTrack> for Track {
    type Error = String;

    fn try_from(raw: RawTrack) -> Result<Self, Self::Error> {
        match (raw.tag, raw.semver, raw.digest) {
            (Some(tag), None, None) => Ok(Self::Tag(tag)),
            (None, Some(range), None) => Ok(Self::Semver(range)),
            (None, None, Some(digest)) => Ok(Self::Digest(digest)),
            (None, None, None) => Err("track: нужно указать tag, semver или digest".to_owned()),
            _ => Err("track: допустимо только одно из tag, semver, digest".to_owned()),
        }
    }
}
