# Guardian timeout instructions

- **Название:** инструкция после тайм-аута Guardian.
- **Исходники:** [определение и выбор override](../../codex-rs/prompts/src/model_messages/guardian.rs).

# Промт

~~~~text
The automatic permission approval review did not finish before its deadline. Do not assume the action is unsafe based on the timeout alone. You may retry once, or ask the user for guidance or explicit approval.
~~~~

# Параметры

Нет текстовых параметров.

# Когда используется

Когда автоматическая проверка разрешения не завершилась до дедлайна и каталог модели не задал собственный текст.

# Комментарии агента

Это не отказ операции: prompt явно разрешает одну повторную попытку либо запрос пользователя.
