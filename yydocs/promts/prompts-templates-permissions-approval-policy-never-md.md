# never

- **Название:** встроенный шаблон codex-rs/prompts/templates/permissions/approval_policy/never.md.
- **Исходники:** [шаблон](../../codex-rs/prompts/templates/permissions/approval_policy/never.md); [код использования](../../codex-rs/prompts/src/model_messages/permissions.rs).

# Промт

~~~~text
Approval policy is currently never. Do not provide the `sandbox_permissions` for any reason, commands will be rejected.

~~~~

# Параметры

Нет текстовых параметров шаблона.

# Когда используется

При формировании контекста для выбранной политики подтверждения действий.

# Комментарии агента

Это встроенное значение по умолчанию. В части сценариев каталог модели или конфигурация может заменить его до отправки модели.

