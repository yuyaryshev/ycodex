# workspace_write

- **Название:** встроенный шаблон codex-rs/prompts/templates/permissions/sandbox_mode/workspace_write.md.
- **Исходники:** [шаблон](../../codex-rs/prompts/templates/permissions/sandbox_mode/workspace_write.md); [код использования](../../codex-rs/prompts/src/model_messages/permissions.rs).

# Промт

~~~~text
Filesystem sandboxing defines which files can be read or written. `sandbox_mode` is `workspace-write`: The sandbox permits reading files, and editing files in `cwd` and `writable_roots`. Editing files in other directories requires approval. Network access is {{ network_access }}.

~~~~

# Параметры

- {{network_access}} — значение подставляется кодом рендера; точка формирования указана в файле использования.

# Когда используется

При формировании контекста для выбранного режима sandbox.

# Комментарии агента

Это встроенное значение по умолчанию. В части сценариев каталог модели или конфигурация может заменить его до отправки модели.

