# realtime_end

- **Название:** встроенный шаблон codex-rs/prompts/templates/realtime/realtime_end.md.
- **Исходники:** [шаблон](../../codex-rs/prompts/templates/realtime/realtime_end.md); [код использования](../../codex-rs/prompts/src/realtime.rs).

# Промт

~~~~text
Realtime conversation ended.

Subsequent user input will return to typed text rather than transcript-style text. Do not assume recognition errors or missing punctuation once realtime has ended. Resume normal chat behavior.

~~~~

# Параметры

Нет текстовых параметров шаблона.

# Когда используется

В realtime-сеансе при его запуске, работе backend и завершении.

# Комментарии агента

Это встроенное значение по умолчанию. В части сценариев каталог модели или конфигурация может заменить его до отправки модели.

