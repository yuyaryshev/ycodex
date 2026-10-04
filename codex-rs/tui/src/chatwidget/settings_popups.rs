//! Settings-adjacent popup surfaces for `ChatWidget`.
//!
//! This keeps theme and experimental-feature UI out of the main
//! orchestration module without changing their event wiring.

use super::*;

impl ChatWidget {
    pub(super) fn open_styles_picker(&mut self) {
        let items = [
            (
                crate::session_styles::StyleTarget::DefaultBackground,
                "Default background color",
                "Color used when a session has no override",
            ),
            (
                crate::session_styles::StyleTarget::DefaultForeground,
                "Default font color",
                "Color used when a session has no override",
            ),
            (
                crate::session_styles::StyleTarget::SessionBackground,
                "This session background color",
                "Saved by this Session ID",
            ),
            (
                crate::session_styles::StyleTarget::SessionForeground,
                "This session font color",
                "Saved by this Session ID",
            ),
        ]
        .into_iter()
        .map(
            |(target, name, description)| crate::bottom_pane::SelectionItem {
                name: name.into(),
                description: Some(description.into()),
                actions: vec![Box::new(move |tx| {
                    tx.send(AppEvent::OpenStyleColorPicker { target });
                })],
                dismiss_on_select: true,
                ..Default::default()
            },
        )
        .collect();
        self.bottom_pane
            .show_selection_view(crate::bottom_pane::SelectionViewParams {
                title: Some("Styles".into()),
                items,
                ..crate::bottom_pane::SelectionViewParams::picker()
            });
    }

    pub(crate) fn open_style_color_picker(&mut self, target: crate::session_styles::StyleTarget) {
        const COLORS: &[(Option<(u8, u8, u8)>, &str)] = &[
            (None, "Terminal default"),
            (Some((0, 0, 0)), "Black"),
            (Some((128, 0, 0)), "Maroon"),
            (Some((0, 100, 0)), "Green"),
            (Some((128, 128, 0)), "Olive"),
            (Some((0, 0, 128)), "Navy"),
            (Some((128, 0, 128)), "Purple"),
            (Some((0, 128, 128)), "Teal"),
            (Some((192, 192, 192)), "Silver"),
            (Some((255, 255, 255)), "White"),
            (Some((255, 140, 0)), "Orange"),
            (Some((220, 20, 60)), "Crimson"),
        ];
        let thread_id = self.thread_id();
        let selected = crate::session_styles::selected_color(target, thread_id);
        let items = COLORS
            .iter()
            .map(|&(color, name)| crate::bottom_pane::SelectionItem {
                name: name.into(),
                is_current: color == selected,
                actions: vec![Box::new(move |tx| {
                    tx.send(AppEvent::StyleColorSelected { target, color });
                })],
                dismiss_on_select: true,
                ..Default::default()
            })
            .collect();
        self.bottom_pane
            .show_selection_view(crate::bottom_pane::SelectionViewParams {
                title: Some("Choose color".into()),
                items,
                ..crate::bottom_pane::SelectionViewParams::picker()
            });
    }

    pub(super) fn open_theme_picker(&mut self) {
        let codex_home = codex_utils_home_dir::find_codex_home().ok();
        let params = crate::theme_picker::build_theme_picker_params(
            self.local_settings.tui.theme.as_deref(),
            codex_home.as_deref(),
            self.last_rendered_width.get(),
        );
        self.bottom_pane.show_selection_view(params);
    }

    pub(crate) fn open_experimental_popup(&mut self) {
        let Some(thread_id) = self.thread_id() else {
            self.add_info_message(
                "Experimental features are unavailable until startup completes.".to_string(),
                /*hint*/ None,
            );
            return;
        };
        let (response_tx, response_rx) = tokio::sync::oneshot::channel();
        self.app_event_tx.send(AppEvent::FetchExperimentalFeatures {
            thread_id,
            response_tx,
        });
        let view = ExperimentalFeaturesView::new(
            Vec::new(),
            thread_id,
            Some(response_rx),
            self.app_event_tx.clone(),
            self.bottom_pane.list_keymap(),
        );
        self.bottom_pane.show_view(Box::new(view));
    }
}
