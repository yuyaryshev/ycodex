# prompt

- **Название:** встроенный шаблон codex-rs/prompts/templates/compact/prompt.md.
- **Исходники:** [шаблон](../../codex-rs/prompts/templates/compact/prompt.md); [код использования](../../codex-rs/prompts/src/compact.rs).

# Промт

~~~~text
You are performing a CONTEXT CHECKPOINT COMPACTION. Create a handoff summary for another LLM that will resume the task.

Include:
- Current progress and key decisions made
- Important context, constraints, or user preferences
- What remains to be done (clear next steps)
- Any critical data, examples, or references needed to continue

Be concise, structured, and focused on helping the next LLM seamlessly continue the work.

~~~~

# Параметры

Нет текстовых параметров шаблона.

# Когда используется

При сжатии истории диалога и формировании её итогового summary.

# Комментарии агента

Это встроенное значение по умолчанию. В части сценариев каталог модели или конфигурация может заменить его до отправки модели.

