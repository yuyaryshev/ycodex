# content-filter guidance

- **Название:** инструкция после блокировки ответа content filter.
- **Исходники:** [определение и выбор fallback](../../codex-rs/prompts/src/model_messages.rs).

# Промт

~~~~text
Your previous response was blocked by a content filter. Do not treat this as a transient failure or try to reproduce or work around the blocked content through repeated attempts, altered formatting, splitting, encoding, tools, subagents, or later wakes. Briefly explain the limitation and offer a permitted alternative. Continue unrelated authorized work.
~~~~

# Параметры

Нет текстовых параметров.

# Когда используется

После блокировки предыдущего ответа фильтром, если допустимый override из каталога модели отсутствует, пуст или превышает 512 байт.

# Комментарии агента

Текст намеренно ограничивает повторные попытки обхода фильтра. Это fallback, а не универсальное правило для всех ответов.

