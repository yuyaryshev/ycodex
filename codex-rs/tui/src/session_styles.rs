//! Persisted default and per-session terminal colors for the local TUI.

use std::collections::BTreeMap;
use std::fs;
use std::sync::LazyLock;
use std::sync::RwLock;

use codex_protocol::ThreadId;
use ratatui::style::Color;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub(crate) struct SessionStyle {
    pub(crate) background: Option<(u8, u8, u8)>,
    pub(crate) foreground: Option<(u8, u8, u8)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StyleTarget {
    DefaultBackground,
    DefaultForeground,
    SessionBackground,
    SessionForeground,
}

static ACTIVE_STYLE: LazyLock<RwLock<SessionStyle>> =
    LazyLock::new(|| RwLock::new(SessionStyle::default()));

#[derive(Default, Deserialize, Serialize)]
pub(crate) struct StoredStyles {
    pub(crate) default: SessionStyle,
    pub(crate) sessions: BTreeMap<String, SessionStyle>,
}

pub(crate) fn style_path() -> Option<std::path::PathBuf> {
    codex_utils_home_dir::find_codex_home()
        .ok()
        .map(|home| home.as_path().join("styles.json"))
}

pub(crate) fn load() -> StoredStyles {
    style_path()
        .and_then(|path| fs::read(path).ok())
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or_default()
}

pub(crate) fn save(styles: &StoredStyles) -> Result<(), String> {
    let Some(path) = style_path() else {
        return Err("could not resolve CODEX_HOME".into());
    };
    let bytes = serde_json::to_vec_pretty(styles).map_err(|err| err.to_string())?;
    fs::write(path, bytes).map_err(|err| err.to_string())
}

pub(crate) fn effective(styles: &StoredStyles, thread_id: Option<ThreadId>) -> SessionStyle {
    let mut effective = styles.default;
    if let Some(session) = thread_id.and_then(|id| styles.sessions.get(&id.to_string())) {
        if session.background.is_some() {
            effective.background = session.background;
        }
        if session.foreground.is_some() {
            effective.foreground = session.foreground;
        }
    }
    effective
}

pub(crate) fn selected_color(
    target: StyleTarget,
    thread_id: Option<ThreadId>,
) -> Option<(u8, u8, u8)> {
    let styles = load();
    match target {
        StyleTarget::DefaultBackground => styles.default.background,
        StyleTarget::DefaultForeground => styles.default.foreground,
        StyleTarget::SessionBackground => thread_id
            .and_then(|id| styles.sessions.get(&id.to_string()))
            .and_then(|style| style.background),
        StyleTarget::SessionForeground => thread_id
            .and_then(|id| styles.sessions.get(&id.to_string()))
            .and_then(|style| style.foreground),
    }
}

pub(crate) fn set_color(
    target: StyleTarget,
    thread_id: Option<ThreadId>,
    color: Option<(u8, u8, u8)>,
) -> Result<(), String> {
    let mut styles = load();
    let style = match target {
        StyleTarget::DefaultBackground | StyleTarget::DefaultForeground => &mut styles.default,
        StyleTarget::SessionBackground | StyleTarget::SessionForeground => {
            let thread_id =
                thread_id.ok_or("session styles are unavailable until startup completes")?;
            styles.sessions.entry(thread_id.to_string()).or_default()
        }
    };
    match target {
        StyleTarget::DefaultBackground | StyleTarget::SessionBackground => style.background = color,
        StyleTarget::DefaultForeground | StyleTarget::SessionForeground => style.foreground = color,
    }
    save(&styles)?;
    set_active_thread(thread_id);
    Ok(())
}

pub(crate) fn set_active_thread(thread_id: Option<ThreadId>) {
    *ACTIVE_STYLE.write().expect("session style lock poisoned") = effective(&load(), thread_id);
}

pub(crate) fn apply(buffer: &mut ratatui::buffer::Buffer) {
    let style = *ACTIVE_STYLE.read().expect("session style lock poisoned");
    let foreground = color(style.foreground);
    let background = color(style.background);
    for cell in &mut buffer.content {
        if cell.fg == Color::Reset
            && let Some(foreground) = foreground
        {
            cell.fg = foreground;
        }
        if cell.bg == Color::Reset
            && let Some(background) = background
        {
            cell.bg = background;
        }
    }
}

pub(crate) fn color(value: Option<(u8, u8, u8)>) -> Option<Color> {
    value.map(|(red, green, blue)| Color::Rgb(red, green, blue))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn session_style_overrides_only_its_configured_color() {
        let thread_id =
            ThreadId::from_string("00000000-0000-0000-0000-000000000123").expect("valid thread id");
        let mut styles = StoredStyles {
            default: SessionStyle {
                background: Some((1, 2, 3)),
                foreground: Some((4, 5, 6)),
            },
            ..Default::default()
        };
        styles.sessions.insert(
            thread_id.to_string(),
            SessionStyle {
                background: Some((7, 8, 9)),
                foreground: None,
            },
        );

        assert_eq!(
            effective(&styles, Some(thread_id)),
            SessionStyle {
                background: Some((7, 8, 9)),
                foreground: Some((4, 5, 6)),
            }
        );
    }
}
