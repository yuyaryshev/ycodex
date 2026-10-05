# request_user_input_async description

- **Название:** описание инструмента асинхронного запроса информации у пользователя.
- **Исходники:** [определение и выбор override](../../codex-rs/prompts/src/model_messages.rs).

# Промт

~~~~text
Ask the user one or more questions during ongoing work. Use this tool only to request missing information, preferences, constraints, clarification, or approval. The tool returns immediately without ending the turn or waiting for a reply; any reply arrives asynchronously as a new user message. Keep questions concise, self-contained, and easy to understand, using a level of detail appropriate to the user and task. The UI always allows a free-text answer, including when suggested options are provided. A preselected option is not submitted automatically.
~~~~

# Параметры

Нет текстовых параметров.

# Когда используется

В описании tool `request_user_input_async`, когда каталог выбранной модели не задаёт собственный description.

# Комментарии агента

Это instruction для модели, а не текст, показанный пользователю. Модельный каталог может заменить её полностью.

