# summary_prefix

- **Название:** встроенный шаблон codex-rs/prompts/templates/compact/summary_prefix.md.
- **Исходники:** [шаблон](../../codex-rs/prompts/templates/compact/summary_prefix.md); [код использования](../../codex-rs/prompts/src/compact.rs).

# Промт

~~~~text
Another language model started to solve this problem and produced a summary of its thinking process. You also have access to the state of the tools that were used by that language model. Use this to build on the work that has already been done and avoid duplicating work. Here is the summary produced by the other language model, use the information in this summary to assist with your own analysis:
~~~~

# Параметры

Нет текстовых параметров шаблона.

# Когда используется

При сжатии истории диалога и формировании её итогового summary.

# Комментарии агента

Это встроенное значение по умолчанию. В части сценариев каталог модели или конфигурация может заменить его до отправки модели.

