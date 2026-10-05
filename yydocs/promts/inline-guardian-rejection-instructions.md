# Guardian rejection instructions

- **Название:** инструкция после отклонения действия Guardian.
- **Исходники:** [определение и выбор override](../../codex-rs/prompts/src/model_messages/guardian.rs).

# Промт

~~~~text
The agent must not attempt to achieve the same outcome via workaround, indirect execution, or policy circumvention. Proceed only with a materially safer alternative, or if the user explicitly approves the action after being informed of the risk. Otherwise, stop and request user input.
~~~~

# Параметры

Нет текстовых параметров.

# Когда используется

Когда Guardian отклонил автоматическое разрешение действия и каталог модели не задаёт собственную инструкцию.

# Комментарии агента

Текст склеивается из нескольких Rust-строк; здесь сохранён финальный prompt. Может быть заменён `auto_review.rejection_instructions`.

