# tool_description

- **Название:** встроенный шаблон codex-rs/core/templates/search_tool/tool_description.md.
- **Исходники:** [шаблон](../../codex-rs/core/templates/search_tool/tool_description.md); [код использования](../../codex-rs/core/src/session/world_state.rs).

# Промт

~~~~text
# Apps (Connectors) tool discovery

Searches over apps/connectors tool metadata with BM25 and exposes matching tools for the next model call.

You have access to all the tools of the following apps/connectors:
{{app_descriptions}}
Some of the tools may not have been provided to you upfront, and you should use this tool (`tool_search`) to search for the required tools and load them for the apps mentioned above. For the apps mentioned above, always use `tool_search` instead of `list_mcp_resources` or `list_mcp_resource_templates` for tool discovery.

~~~~

# Параметры

- {{app_descriptions}} — значение подставляется кодом рендера; точка формирования указана в файле использования.

# Когда используется

При описании инструментов поиска и установки plugin для модели.

# Комментарии агента

Это встроенное значение по умолчанию. В части сценариев каталог модели или конфигурация может заменить его до отправки модели.

