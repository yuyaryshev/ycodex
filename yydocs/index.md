# Библиотека встроенных prompt-шаблонов

Эта библиотека фиксирует **55** статических текстов, которые ycodex может передавать модели: файловые шаблоны и пять коротких fallback-инструкций, определённых непосредственно в Rust.

Не включены пользовательские сообщения, AGENTS.md, навыки, плагины, данные MCP, тексты тестов и контекстные фрагменты, которые целиком формируются из внешних данных. Они не являются зашитыми prompt-значениями.

Каждый документ содержит точный bundled-шаблон на момент ревизии 351abb571, ссылку на первичный исходник и место основного рендера. В ряде случаев модельный каталог может заменить bundled-текст до отправки модели.

| Категория | Название | Когда используется | Файл |
| --- | --- | --- | --- |
| Collaboration | default | Когда выбран соответствующий режим collaboration: Default либо Plan. | [открыть](promts/collaboration-mode-templates-templates-default-md.md) |
| Collaboration | plan | Когда выбран соответствующий режим collaboration: Default либо Plan. | [открыть](promts/collaboration-mode-templates-templates-plan-md.md) |
| Model instructions | gpt-5.1-codex-max_prompt | Как базовые model instructions для соответствующей модели или её fallback-конфигурации. | [открыть](promts/core-gpt-5-1-codex-max-prompt-md.md) |
| Model instructions | gpt_5_1_prompt | Как базовые model instructions для соответствующей модели или её fallback-конфигурации. | [открыть](promts/core-gpt-5-1-prompt-md.md) |
| Model instructions | gpt-5.2-codex_prompt | Как базовые model instructions для соответствующей модели или её fallback-конфигурации. | [открыть](promts/core-gpt-5-2-codex-prompt-md.md) |
| Model instructions | gpt_5_2_prompt | Как базовые model instructions для соответствующей модели или её fallback-конфигурации. | [открыть](promts/core-gpt-5-2-prompt-md.md) |
| Model instructions | gpt_5_codex_prompt | Как базовые model instructions для соответствующей модели или её fallback-конфигурации. | [открыть](promts/core-gpt-5-codex-prompt-md.md) |
| Agents | orchestrator | Для встроенной роли оркестратора при многоагентной работе. | [открыть](promts/core-templates-agents-orchestrator-md.md) |
| Collaboration | experimental_prompt | В экспериментальном многоагентном сценарии collaboration. | [открыть](promts/core-templates-collab-experimental-prompt-md.md) |
| Model instructions | gpt-5.2-codex_instructions_template | При построении системного контекста выбранной модели. | [открыть](promts/core-templates-model-instructions-gpt-5-2-codex-instructions-template-md.md) |
| Model instructions | gpt-5.2-codex_friendly | При выборе соответствующей personality для gpt-5.2-codex. | [открыть](promts/core-templates-personalities-gpt-5-2-codex-friendly-md.md) |
| Model instructions | gpt-5.2-codex_pragmatic | При выборе соответствующей personality для gpt-5.2-codex. | [открыть](promts/core-templates-personalities-gpt-5-2-codex-pragmatic-md.md) |
| Review | history_message_completed | При запуске review-подзадачи или сохранении её результата в истории. | [открыть](promts/core-templates-review-history-message-completed-md.md) |
| Review | history_message_interrupted | При запуске review-подзадачи или сохранении её результата в истории. | [открыть](promts/core-templates-review-history-message-interrupted-md.md) |
| Tool descriptions | request_plugin_install_description | При описании инструментов поиска и установки plugin для модели. | [открыть](promts/core-templates-search-tool-request-plugin-install-description-md.md) |
| Tool descriptions | tool_description | При описании инструментов поиска и установки plugin для модели. | [открыть](promts/core-templates-search-tool-tool-description-md.md) |
| Goals | budget_limit | При продолжении цели, изменении её objective или достижении token budget. | [открыть](promts/ext-goal-templates-goals-budget-limit-md.md) |
| Goals | continuation | При продолжении цели, изменении её objective или достижении token budget. | [открыть](promts/ext-goal-templates-goals-continuation-md.md) |
| Goals | objective_updated | При продолжении цели, изменении её objective или достижении token budget. | [открыть](promts/ext-goal-templates-goals-objective-updated-md.md) |
| Memories | read_path | Когда включён инструмент Memories и найдена сохранённая memory summary. | [открыть](promts/ext-memories-templates-memories-read-path-md.md) |
| Memories | read_path_v2 | Когда включён инструмент Memories и найдена сохранённая memory summary. | [открыть](promts/ext-memories-templates-memories-read-path-v2-md.md) |
| Inline fallback | content-filter guidance | После блокировки предыдущего ответа фильтром, если допустимый override из каталога модели отсутствует, пуст или превышает 512 байт. | [открыть](promts/inline-content-filter-guidance.md) |
| Inline fallback | context-window reminder | Когда рантайм считает, что контекст выбранной модели близок к лимиту, а каталог модели не подменил сообщение. | [открыть](promts/inline-context-window-reminder.md) |
| Inline fallback | Guardian rejection instructions | Когда Guardian отклонил автоматическое разрешение действия и каталог модели не задаёт собственную инструкцию. | [открыть](promts/inline-guardian-rejection-instructions.md) |
| Inline fallback | Guardian timeout instructions | Когда автоматическая проверка разрешения не завершилась до дедлайна и каталог модели не задал собственный текст. | [открыть](promts/inline-guardian-timeout-instructions.md) |
| Inline fallback | request_user_input_async description | В описании tool `request_user_input_async`, когда каталог выбранной модели не задаёт собственный description. | [открыть](promts/inline-request-user-input-async-description.md) |
| Memories | instructions | Во внутреннем конвейере извлечения и консолидации долговременной памяти. | [открыть](promts/memories-write-templates-extensions-ad-hoc-instructions-md.md) |
| Memories | consolidation | Во внутреннем конвейере извлечения и консолидации долговременной памяти. | [открыть](promts/memories-write-templates-memories-consolidation-md.md) |
| Memories | consolidation_v2 | Во внутреннем конвейере извлечения и консолидации долговременной памяти. | [открыть](promts/memories-write-templates-memories-consolidation-v2-md.md) |
| Memories | stage_one_input | Во внутреннем конвейере извлечения и консолидации долговременной памяти. | [открыть](promts/memories-write-templates-memories-stage-one-input-md.md) |
| Memories | stage_one_input_v2 | Во внутреннем конвейере извлечения и консолидации долговременной памяти. | [открыть](promts/memories-write-templates-memories-stage-one-input-v2-md.md) |
| Memories | stage_one_system | Во внутреннем конвейере извлечения и консолидации долговременной памяти. | [открыть](promts/memories-write-templates-memories-stage-one-system-md.md) |
| Memories | stage_one_system_v2 | Во внутреннем конвейере извлечения и консолидации долговременной памяти. | [открыть](promts/memories-write-templates-memories-stage-one-system-v2-md.md) |
| Model instructions | prompt | Как базовые model instructions для соответствующей модели или её fallback-конфигурации. | [открыть](promts/models-manager-prompt-md.md) |
| Runtime | prompt | При сжатии истории диалога и формировании её итогового summary. | [открыть](promts/prompts-templates-compact-prompt-md.md) |
| Runtime | summary_prefix | При сжатии истории диалога и формировании её итогового summary. | [открыть](promts/prompts-templates-compact-summary-prefix-md.md) |
| Guardian | classifier_instructions | Во время автоматической проверки разрешений (Guardian) или классификации действия. | [открыть](promts/prompts-templates-guardian-classifier-instructions-md.md) |
| Guardian | node_repl_policy | Во время автоматической проверки разрешений (Guardian) или классификации действия. | [открыть](promts/prompts-templates-guardian-node-repl-policy-md.md) |
| Guardian | policy | Во время автоматической проверки разрешений (Guardian) или классификации действия. | [открыть](promts/prompts-templates-guardian-policy-md.md) |
| Guardian | policy_template | Во время автоматической проверки разрешений (Guardian) или классификации действия. | [открыть](promts/prompts-templates-guardian-policy-template-md.md) |
| Permissions | never | При формировании контекста для выбранной политики подтверждения действий. | [открыть](promts/prompts-templates-permissions-approval-policy-never-md.md) |
| Permissions | on_request | При формировании контекста для выбранной политики подтверждения действий. | [открыть](promts/prompts-templates-permissions-approval-policy-on-request-md.md) |
| Permissions | on_request_rule_request_permission | При формировании контекста для выбранной политики подтверждения действий. | [открыть](promts/prompts-templates-permissions-approval-policy-on-request-rule-request-permission-md.md) |
| Permissions | unless_trusted | При формировании контекста для выбранной политики подтверждения действий. | [открыть](promts/prompts-templates-permissions-approval-policy-unless-trusted-md.md) |
| Permissions | danger_full_access | При формировании контекста для выбранного режима sandbox. | [открыть](promts/prompts-templates-permissions-sandbox-mode-danger-full-access-md.md) |
| Permissions | read_only | При формировании контекста для выбранного режима sandbox. | [открыть](promts/prompts-templates-permissions-sandbox-mode-read-only-md.md) |
| Permissions | workspace_write | При формировании контекста для выбранного режима sandbox. | [открыть](promts/prompts-templates-permissions-sandbox-mode-workspace-write-md.md) |
| Runtime | persistent_mode | Когда активирован persistent mode; текст добавляется к model instructions. | [открыть](promts/prompts-templates-persistent-mode-md.md) |
| Realtime | backend_prompt | В realtime-сеансе при его запуске, работе backend и завершении. | [открыть](promts/prompts-templates-realtime-backend-prompt-md.md) |
| Realtime | realtime_end | В realtime-сеансе при его запуске, работе backend и завершении. | [открыть](promts/prompts-templates-realtime-realtime-end-md.md) |
| Realtime | realtime_start | В realtime-сеансе при его запуске, работе backend и завершении. | [открыть](promts/prompts-templates-realtime-realtime-start-md.md) |
| Review | exit_interrupted | При запуске review-подзадачи или сохранении её результата в истории. | [открыть](promts/prompts-templates-review-exit-interrupted-xml.md) |
| Review | exit_success | При запуске review-подзадачи или сохранении её результата в истории. | [открыть](promts/prompts-templates-review-exit-success-xml.md) |
| Review | rubric | При запуске review-подзадачи или сохранении её результата в истории. | [открыть](promts/prompts-templates-review-rubric-md.md) |
| Model instructions | default | Как базовые model instructions для соответствующей модели или её fallback-конфигурации. | [открыть](promts/protocol-src-prompts-base-instructions-default-md.md) |

