//! Описание registry и способа авторизации.

use serde::Deserialize;

/// Тип registry; определяет схему авторизации.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    /// Docker Hub.
    Dockerhub,
    /// GitHub Container Registry.
    Ghcr,
    /// Gitea.
    Gitea,
    /// Forgejo.
    Forgejo,
}

/// Registry, к которому обращается утилита.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Registry {
    /// Имя для ссылок из политики.
    pub name: String,
    /// Тип registry.
    pub kind: Kind,
    /// Хост (обязателен для Gitea и Forgejo, проверяется валидацией).
    pub host: Option<String>,
    /// Переменная окружения с именем пользователя.
    pub username_env: Option<String>,
    /// Переменная окружения с токеном.
    pub token_env: Option<String>,
}
