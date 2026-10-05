# exit_success

- **Название:** встроенный шаблон codex-rs/prompts/templates/review/exit_success.xml.
- **Исходники:** [шаблон](../../codex-rs/prompts/templates/review/exit_success.xml); [код использования](../../codex-rs/prompts/src/review_request.rs).

# Промт

~~~~text
<user_action>
  <context>User initiated a review task. Here's the full review output from reviewer model. User may select one or more comments to resolve.</context>
  <action>review</action>
  <results>
  {{results}}
  </results>
  </user_action>

~~~~

# Параметры

- {{results}} — значение подставляется кодом рендера; точка формирования указана в файле использования.

# Когда используется

При запуске review-подзадачи или сохранении её результата в истории.

# Комментарии агента

Это встроенное значение по умолчанию. В части сценариев каталог модели или конфигурация может заменить его до отправки модели.

