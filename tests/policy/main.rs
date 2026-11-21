//! Интеграционные тесты модуля policy: один test-crate, файлы повторяют структуру `src/policy`.

// `cfg(test)` на модулях нужен, чтобы clippy считал весь код тестовым (unwrap/expect/panic разрешены).
#[cfg(test)]
mod document;
#[cfg(test)]
mod interval;
#[cfg(test)]
mod load;
#[cfg(test)]
mod track;
#[cfg(test)]
mod validate;
