//! Интервал времени в политике: целое число и единица `s`, `m` или `h`.

use std::time::Duration;

use serde::{Deserialize, Deserializer};

/// Интервал, заданный строкой вида `30s`, `5m`, `2h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Interval(pub Duration);

impl Interval {
    /// Создаёт интервал из секунд.
    #[must_use]
    pub const fn from_secs(secs: u64) -> Self {
        Self(Duration::from_secs(secs))
    }

    /// Разбирает строку вида `30s`, `5m`, `2h`.
    ///
    /// # Errors
    /// Возвращает описание ошибки, если строка не соответствует формату
    /// или значение не помещается в 64 бита секунд.
    pub fn parse(text: &str) -> Result<Self, String> {
        let (index, unit) = text.char_indices().last().ok_or("пустой интервал")?;
        let factor = unit_seconds(unit)
            .ok_or_else(|| format!("неизвестная единица `{unit}` в `{text}`, допустимы s, m, h"))?;
        let digits = &text[..index];
        if digits.is_empty() || !digits.chars().all(|c| c.is_ascii_digit()) {
            return Err(format!("`{text}`: ожидается целое число перед единицей"));
        }
        let count: u64 = digits.parse().map_err(|_| too_large(text))?;
        count
            .checked_mul(factor)
            .map(Self::from_secs)
            .ok_or_else(|| too_large(text))
    }
}

fn unit_seconds(unit: char) -> Option<u64> {
    match unit {
        's' => Some(1),
        'm' => Some(60),
        'h' => Some(3600),
        _ => None,
    }
}

fn too_large(text: &str) -> String {
    format!("`{text}`: слишком большое значение")
}

impl<'de> Deserialize<'de> for Interval {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        Self::parse(&text).map_err(serde::de::Error::custom)
    }
}
