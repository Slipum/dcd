#!/usr/bin/env bash
# Проверка длины Rust-файлов: > LIMIT строк = ошибка, >= WARN = предупреждение.
# Считаются все строки, включая пустые и комментарии.
set -euo pipefail

LIMIT="${LIMIT:-200}"
WARN="${WARN:-180}"
status=0

while IFS= read -r file; do
  lines=$(wc -l < "$file" | tr -d ' ')
  if [ "$lines" -gt "$LIMIT" ]; then
    echo "ERROR: $file: $lines строк (лимит $LIMIT)"
    status=1
  elif [ "$lines" -ge "$WARN" ]; then
    echo "WARN:  $file: $lines строк (близко к лимиту $LIMIT)"
  fi
done < <(find src tests -name '*.rs' 2>/dev/null | sort)

exit "$status"
