# history_message_completed

- **Название:** встроенный шаблон codex-rs/core/templates/review/history_message_completed.md.
- **Исходники:** [шаблон](../../codex-rs/core/templates/review/history_message_completed.md); [код использования](../../codex-rs/core/src/session/world_state.rs).

# Промт

~~~~text
<user_action>
  <context>User initiated a review task. Here's the full review output from reviewer model. User may select one or more comments to resolve.</context>
  <action>review</action>
  <results>
  {findings}
  </results>
</user_action>


~~~~

# Параметры

Нет текстовых параметров шаблона.

# Когда используется

При запуске review-подзадачи или сохранении её результата в истории.

# Комментарии агента

Это встроенное значение по умолчанию. В части сценариев каталог модели или конфигурация может заменить его до отправки модели.

