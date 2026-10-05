# context-window reminder

- **Название:** напоминание об исчерпании окна контекста.
- **Исходники:** [определение и выбор override](../../codex-rs/prompts/src/model_messages.rs).

# Промт

~~~~text
Your context window is nearly exhausted (only {n_remaining} tokens remaining) and will be automatically reset for you soon. Once reset, message items in current context window will be cleared in the new window, but notes and history items will be persistent across windows.
~~~~

# Параметры

- `{n_remaining}` — расчётное число токенов, оставшихся до автоматического сброса окна.

# Когда используется

Когда рантайм считает, что контекст выбранной модели близок к лимиту, а каталог модели не подменил сообщение.

# Комментарии агента

Это одна склеенная в Rust строка `concat!`; в документе она приведена в окончательном виде.

