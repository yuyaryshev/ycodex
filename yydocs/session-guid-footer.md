# Session GUID in the TUI footer

## Purpose

The interactive lower panel now shows `Session ID: <GUID>` after the session history metadata is initialized. If the panel is also showing an active-agent label, both are kept in the same contextual footer line.

The identifier is hidden while the footer needs to prioritize interactive instructions such as shortcuts, history search, or quit confirmation.

## Scope

- `codex-rs/tui/src/bottom_pane/chat_composer.rs`
- `codex-rs/tui/src/bottom_pane/chat_composer/footer_state.rs`

## Upstream PR artifact

This is independent of daemon behavior and can be submitted as a small TUI-only pull request, including a footer snapshot that shows the session ID.
