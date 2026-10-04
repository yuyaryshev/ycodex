# Shared profile with isolated ycodex runtimes

`codex`, `ycodex`, and `ycodex_next` deliberately use a common logical profile while
keeping their daemon state and SQLite stores independent.  This avoids cross-version
daemon ownership and SQLite WAL contention without creating separate credentials or
MCP configuration.

## Layout

```text
%USERPROFILE%\\.codex                 ordinary Codex and shared data
%USERPROFILE%\\.ycodex-overlay         ycodex runtime overlay
%USERPROFILE%\\.ycodex-next-overlay    ycodex_next runtime overlay
```

The two overlays contain NTFS symbolic links to the shared profile for authentication,
configuration, MCP/OAuth data, plugins, rules, skills, sandbox configuration, session
rollouts, the session index, prompt history, writer locks, and `styles.json`.

Each overlay owns its `app-server-control`, `app-server-daemon`, `packages`, temporary
directories, `node_repl`, daemon logs, and every SQLite database (including its
`-wal` and `-shm` companions).  The SQLite files are never linked or hard-linked.

## Consequences

`ycodex resume <session-id>` can read the common rollout under `sessions/`; therefore
a session identifier remains usable from all three commands.  The local state database
may rebuild its metadata from that rollout when it first sees the session.

SQLite-only presentation and runtime metadata can differ between commands: recent-chat
ordering, archived/section state, active goals, durable queued items, memories, and
logs are intentionally local.  Do not open the same session id for writing in two
commands at once.  The shared `thread-writer-locks` directory protects against that,
but it is not a collaboration mechanism.

## Maintenance rule

Do not replace a shared symbolic-link file with a private copy.  In particular, if a
future configuration writer performs an atomic replace of `config.toml` or
`styles.json`, recreate its link to `%USERPROFILE%\\.codex` before using another
command.  The launch wrappers themselves do not start PowerShell or perform setup, so
they cannot create visible console windows.

The ordinary `codex` command keeps its original `%USERPROFILE%\\.codex` profile and is
not restarted or otherwise changed by this layout.

On the first ordinary `ycodex` or `ycodex_next` launch, its daemon must construct local
SQLite indexes from the shared rollouts.  The wrappers start that daemon and wait for its
control socket for up to three minutes before opening the TUI.  They skip this preflight
for `--version`, `--help`, `app-server`, and explicit `--no-daemon` commands.
