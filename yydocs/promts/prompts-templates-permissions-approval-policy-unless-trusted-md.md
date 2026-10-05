# unless_trusted

- **Название:** встроенный шаблон codex-rs/prompts/templates/permissions/approval_policy/unless_trusted.md.
- **Исходники:** [шаблон](../../codex-rs/prompts/templates/permissions/approval_policy/unless_trusted.md); [код использования](../../codex-rs/prompts/src/model_messages/permissions.rs).

# Промт

~~~~text
 `approval_policy` is `unless-trusted`: The harness will require user approval before running commands unless an explicit exec policy rule allows them.

~~~~

# Параметры

Нет текстовых параметров шаблона.

# Когда используется

При формировании контекста для выбранной политики подтверждения действий.

# Комментарии агента

Это встроенное значение по умолчанию. В части сценариев каталог модели или конфигурация может заменить его до отправки модели.

