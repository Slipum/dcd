# Архитектура и структура проекта

## Принципы
- Слои: `cli` → `engine` → (`policy`, `registry`, `docker`, `git`). Обратных зависимостей нет.
- Ядро решений (`engine/decide.rs`) — чистая функция без ввода-вывода: основной объект тестирования идемпотентности.
- Всё внешнее (Docker, registry, Git, часы) вызывается через трейты, объявленные в слое-потребителе (`engine/ports.rs`), чтобы тестировать на mock.
- Без `mod.rs`: модуль `foo` = файл `foo.rs` (список `pub mod`, документация и при необходимости небольшой фасад) + папка `foo/`.
- «Однотипные компоненты»: папка = одна подсистема, все файлы в ней одной роли (например, «команды CLI» или «шаги движка»). Смешивать роли в одной папке нельзя.
- Тесты лежат в `tests/` и повторяют структуру `src/`: `tests/<модуль>/main.rs` — отдельный test-crate на модуль, дальше файлы по структуре модуля. Тестируется только публичный API. Подмодули в корне test-crate подключаются с `#[cfg(test)]`, чтобы clippy разрешал в них unwrap/expect/panic.

## Структура `src/`
| Путь | Роль | Обоснование |
|------|------|-------------|
| `main.rs` | запуск | только вызов библиотеки и код выхода |
| `lib.rs` | корень | список модулей |
| `error.rs` | типы ошибок | единый тип ошибки с кодами выхода (UC-11) |
| `output.rs` | вывод | единственное место с `print*` (таблицы, JSON) |
| `cli/` | команды | один файл = одна команда: `init`, `validate`, `check`, `diff`, `apply`, `approve`, `rollback`, `status`, `daemon`; `args.rs` парсинг clap |
| `policy/types/` | общие значения | `interval`, `track` |
| `policy/sections/` | секции верхнего уровня | `git`, `registry`, `webhook`, `state` |
| `policy/stacks/` | иерархия стека | `stack`, `service`, `healthcheck` |
| `policy/validate/` | правила согласованности | фасад `validate.rs`; `sections/` и `stacks/` — по файлу правил на тип схемы |
| `policy/` (корень) | | `document` (корневой документ), `load` (чтение YAML) |
| `registry/` | клиенты registry | `oci.rs` единый клиент OCI; `auth/` по файлу на `dockerhub`, `ghcr`, `gitea`, `forgejo`; `platform.rs` выбор манифеста платформы |
| `docker/` | обёртки Docker | `info.rs`, `inspect.rs` (bollard); `compose.rs` (CLI); `override_file.rs` (детерминированный override); `labels.rs` |
| `engine/` | шаги движка | `decide.rs` (чистая), `reconcile.rs`, `health.rs`, `rollback.rs`, `lock.rs`, `ports.rs` |
| `git/` | Git | `sync.rs`, `cache.rs` |
| `trigger/` | триггеры | `poll.rs`, `webhook.rs`, `debounce.rs` (`stable_for`) |

Каждый файл ≤ 200 строк; если файл растёт, выделяется новая подпапка однотипных компонентов (например, `engine/decide/` с правилами решения по одному файлу).

## Порядок разработки (модуль за модулем, тесты после каждого)
1. `policy` модель и загрузка: классы эквивалентности по полям (валидные, пустые, неизвестные, неверные типы).
2. `policy/validate`: правила согласованности (healthcheck none + blue_green и т.п.).
3. `docker/labels` и `docker/override_file`: детерминированность.
4. `engine/decide`: полная таблица классов из `idempotency.md`.
5. `registry`: клиент OCI, выбор платформы (mock HTTP).
6. `docker` обёртки (bollard, compose CLI): за трейтами.
7. `engine` reconcile, health, rollback, lock.
8. `cli` команды и `output`.
9. `git`, затем `trigger` (poll, debounce, webhook).
10. Интеграционные тесты с реальным Docker в CI (Linux).

## Линтер: что чем обеспечивается
| Требование | Механизм |
|------------|----------|
| Функция ≤ 30 строк | `clippy::too_many_lines` (порог в `clippy.toml`), уровень deny |
| Файл ≤ 200 строк | `scripts/check-lengths.sh` (clippy такого lint не имеет); pre-commit и CI |
| Обоснованность файла/папки | ревью + правила этого документа; автоматически не проверяется |
| Нет unwrap/expect/panic вне тестов | clippy deny, исключения для тестов в `clippy.toml` |
| Нельзя обойти правила | `allow_attributes` deny: только `#[expect(..., reason = "...")]` |
| Нет `unsafe` | `unsafe_code = "forbid"` |

**Расхождение:** `too_many_lines` не считает пустые строки и комментарии внутри функции (только строки кода), а файлы скрипт считает полностью. Это мягче, чем «считать всё», для функций. Если нужен строгий счёт, потребуется собственная проверка на `syn`.
