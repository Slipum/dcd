//! Настройки приёма вебхуков от registry.

use std::net::SocketAddr;

use serde::Deserialize;

/// HTTP-слушатель вебхуков.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Webhook {
    /// Адрес и порт, например `127.0.0.1:9000`.
    pub listen: SocketAddr,
    /// Переменная окружения с секретом для проверки подписи.
    pub secret_env: String,
}
