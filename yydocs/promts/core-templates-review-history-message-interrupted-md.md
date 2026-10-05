# history_message_interrupted

- **Название:** встроенный шаблон codex-rs/core/templates/review/history_message_interrupted.md.
- **Исходники:** [шаблон](../../codex-rs/core/templates/review/history_message_interrupted.md); [код использования](../../codex-rs/core/src/session/world_state.rs).

# Промт

~~~~text
<user_action>
  <context>User initiated a review task, but was interrupted. If user asks about this, tell them to re-initiate a review with `/review` and wait for it to complete.</context>
  <action>review</action>
  <results>
  None.
  </results>
</user_action>


~~~~

# Параметры

Нет текстовых параметров шаблона.

# Когда используется

При запуске review-подзадачи или сохранении её результата в истории.

# Комментарии агента

Это встроенное значение по умолчанию. В части сценариев каталог модели или конфигурация может заменить его до отправки модели.

