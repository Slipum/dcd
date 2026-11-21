//! Корневой документ политики.

use serde::Deserialize;

use super::sections::git::Repo;
use super::sections::registry::Registry;
use super::sections::state::State;
use super::sections::webhook::Webhook;
use super::stacks::stack::Stack;

/// Политика целиком.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    /// Версия схемы.
    pub version: u32,
    /// Git-источник.
    pub git: Option<Repo>,
    /// Registry.
    #[serde(default)]
    pub registries: Vec<Registry>,
    /// Стеки.
    #[serde(default)]
    pub stacks: Vec<Stack>,
    /// Приём вебхуков.
    pub webhook: Option<Webhook>,
    /// История деплоев.
    pub state: Option<State>,
}
