# UC-05-health-rollback Провал healthcheck и автооткат

**Актор:** система. **Триггер:** healthcheck не стал healthy за `timeout`.
**Поток:** 1) пометить деплой неуспешным. 2) Если `rollback.on_failure` и есть previous digest — вернуть сервис на previous через override, записав в label `dcd.rejected` провальный digest. 3) Дождаться healthcheck. 4) Вывести ошибку и причину, код выхода ≠ 0.
**Защита от петли:** пока в registry тот же rejected digest, daemon и `apply` его пропускают (статус `skipped: rejected`). Как только появится другой digest, rejected сбрасывается. Принудительная повторная попытка: `dcd apply --retry-rejected`.
**Альтернативы:** healthcheck `none` — откат невозможен, утилита предупреждает на validate/apply; откат тоже провалился — статус critical, сервис оставлен как есть, подробности в логе; previous образ удалён с хоста — повторный pull по digest.
**Результат:** сервис на последней рабочей версии либо явный critical.
