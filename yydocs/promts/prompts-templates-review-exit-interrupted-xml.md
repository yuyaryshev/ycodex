# exit_interrupted

- **Название:** встроенный шаблон codex-rs/prompts/templates/review/exit_interrupted.xml.
- **Исходники:** [шаблон](../../codex-rs/prompts/templates/review/exit_interrupted.xml); [код использования](../../codex-rs/prompts/src/review_request.rs).

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

