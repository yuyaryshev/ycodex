# danger_full_access

- **Название:** встроенный шаблон codex-rs/prompts/templates/permissions/sandbox_mode/danger_full_access.md.
- **Исходники:** [шаблон](../../codex-rs/prompts/templates/permissions/sandbox_mode/danger_full_access.md); [код использования](../../codex-rs/prompts/src/model_messages/permissions.rs).

# Промт

~~~~text
Filesystem sandboxing defines which files can be read or written. `sandbox_mode` is `danger-full-access`: No filesystem sandboxing - all commands are permitted. Network access is {{ network_access }}.

~~~~

# Параметры

- {{network_access}} — значение подставляется кодом рендера; точка формирования указана в файле использования.

# Когда используется

При формировании контекста для выбранного режима sandbox.

# Комментарии агента

Это встроенное значение по умолчанию. В части сценариев каталог модели или конфигурация может заменить его до отправки модели.

