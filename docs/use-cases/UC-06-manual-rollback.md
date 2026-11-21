# UC-06-manual-rollback Ручной откат

**Актор:** оператор. **Триггер:** `dcd rollback <stack>/<service>`.
**Поток:** читает previous digest из labels, подтверждает (TTY) и выполняет UC-03 с этим digest, менять местами current/previous.
**Альтернативы:** previous digest отсутствует — ошибка; `--digest` позволяет указать любой доступный digest.
**Результат:** сервис на выбранной версии.
