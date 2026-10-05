# Windows daemon elevation bypass

## Purpose

Adds `--bypass-safety-y` for this customized Codex build. On Windows it permits the local shared app-server daemon to start when the invoking terminal is elevated.

The upstream protection remains the default. The bypass is process-local and must be explicitly supplied for every Codex invocation.

## Scope

- `codex-rs/tui/src/cli.rs`: exposes the flag for interactive, `resume`, and `fork` commands.
- `codex-rs/cli/src/main.rs`: preserves the flag while merging `resume` and `fork` arguments and enables the bypass before daemon startup.
- `codex-rs/app-server-daemon/src/backend/windows.rs`: skips only the elevation check; all other daemon checks remain active.

## Upstream PR artifact

This feature is self-contained in the files above. An upstream proposal should frame it as an explicitly named, Windows-only escape hatch and retain the default safety behavior.
