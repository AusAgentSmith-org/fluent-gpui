use std::{borrow::Cow, sync::Arc};

use gpui::{AssetSource, Result, SharedString};

const ADD: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M7 2h2v5h5v2H9v5H7V9H2V7h5z"/></svg>"#;
const ARROW_FORWARD: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M9.3 3.3 14 8l-4.7 4.7-1.4-1.4L10.2 9H2V7h8.2L7.9 4.7z"/></svg>"#;
const CHECKMARK: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="m6.2 11.4-3.3-3.3 1.4-1.4 1.9 1.9 5.5-5.5 1.4 1.4z"/></svg>"#;
const CHEVRON_DOWN: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M4.2 6 8 9.8 11.8 6l1.2 1.2-5 5-5-5z"/></svg>"#;
const CHEVRON_UP: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M4.2 10 8 6.2l3.8 3.8 1.2-1.2-5-5-5 5z"/></svg>"#;
const CLEAR: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M4.3 3 8 6.7 11.7 3 13 4.3 9.3 8l3.7 3.7-1.3 1.3L8 9.3 4.3 13 3 11.7 6.7 8 3 4.3z"/></svg>"#;
const COPY: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M5 2h7v9H5zm-2 3h1v7h6v1H3z"/></svg>"#;
const CUT: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M4 5a2 2 0 1 1 1.4-.6L8 7l2.6-2.6A2 2 0 1 1 12 5a2 2 0 0 1-.7-.1L9.4 8l1.9 3.1A2 2 0 1 1 10 12l-2-2-2 2a2 2 0 1 1-1.3-.9L6.6 8 4.7 4.9A2 2 0 0 1 4 5z"/></svg>"#;
const DISMISS: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="m4.3 3 3.7 3.7L11.7 3 13 4.3 9.3 8l3.7 3.7-1.3 1.3L8 9.3 4.3 13 3 11.7 6.7 8 3 4.3z"/></svg>"#;
const FOLDER: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M2 4h5l1 1h6v7.5A1.5 1.5 0 0 1 12.5 14h-9A1.5 1.5 0 0 1 2 12.5z"/></svg>"#;
const GRID: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M2 2h5v5H2zm7 0h5v5H9zM2 9h5v5H2zm7 0h5v5H9z"/></svg>"#;
const LIGHT: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M8 4a4 4 0 1 1 0 8A4 4 0 0 1 8 4zm0-3h1v2H7V1zm0 12h1v2H7v-2zM1 7h2v2H1zm12 0h2v2h-2zM3 2.3 4.4 3.7 3.7 4.4 2.3 3zm9.3 9.3 1.4 1.4-.7.7-1.4-1.4zM13.7 3l-1.4 1.4-.7-.7L13 2.3zM4.4 12.3 3 13.7l-.7-.7 1.4-1.4z"/></svg>"#;
const LIST_TREE: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M3 2h2v2H3zm4 .5h7v1H7zM3 7h2v2H3zm4 .5h7v1H7zM3 12h2v2H3zm4 .5h7v1H7z"/></svg>"#;
const MINIMIZE: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M3 8h10v1.5H3z"/></svg>"#;
const MAXIMIZE: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M3 3h10v10H3zm1.5 1.5v7h7v-7z"/></svg>"#;
const PANEL_OPEN: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M2 3h12v10H2zm2 1.5v7h2.5v-7zm4 0v7h4.5v-7z"/></svg>"#;
const PASTE: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M6 1h4l.5 1H13v12H3V2h2.5zm0 2v1h4V3zM5 6v6h6V6z"/></svg>"#;
const PLUG: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M6 1h1v4h2V1h1v4h1v3a4 4 0 0 1-3 3.9V15H7v-3.1A4 4 0 0 1 4 8V5h2z"/></svg>"#;
const SEARCH: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M7 2a5 5 0 0 1 3.9 8.1l3 3-1.4 1.4-3-3A5 5 0 1 1 7 2zm0 2a3 3 0 1 0 0 6 3 3 0 0 0 0-6z"/></svg>"#;
const SETTINGS: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M7 1h2l.4 2 1.7.7 1.7-1.1 1.4 1.4-1.1 1.7.7 1.7 2 .4v2l-2 .4-.7 1.7 1.1 1.7-1.4 1.4-1.7-1.1-1.7.7-.4 2H7l-.4-2-1.7-.7-1.7 1.1-1.4-1.4 1.1-1.7-.7-1.7-2-.4V8l2-.4.7-1.7-1.1-1.7 1.4-1.4 1.7 1.1 1.7-.7zM8 6a2 2 0 1 0 0 4 2 2 0 0 0 0-4z"/></svg>"#;
const SPINNER: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M8 1a7 7 0 0 1 7 7h-2a5 5 0 0 0-5-5z"/></svg>"#;
const DOCUMENT: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M3 2h7l3 3v9H3V2zm7 0v3h3"/></svg>"#;
const DOWNLOAD: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M8 11 4 7h2.5V2h3v5H12zm-6 2h12v1.5H2z"/></svg>"#;
const UPLOAD: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M8 2l4 4H9.5v5h-3V6H4zm-6 11h12v1.5H2z"/></svg>"#;
const GENERIC: &[u8] = br#"<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path fill="currentColor" d="M3 3h10v10H3z"/></svg>"#;

/// Built-in FluentGUI assets plus an optional application asset fallback.
#[derive(Default)]
pub struct FluentAssets {
    fallback: Option<Arc<dyn AssetSource>>,
}

impl FluentAssets {
    pub fn new(fallback: Option<Arc<dyn AssetSource>>) -> Self {
        Self { fallback }
    }

    fn builtin(path: &str) -> Option<&'static [u8]> {
        Some(match path {
            "icons/add.svg" | "icons/folder_add.svg" => ADD,
            "icons/arrow_forward.svg" => ARROW_FORWARD,
            "icons/checkmark.svg" => CHECKMARK,
            "icons/chevron_down.svg" => CHEVRON_DOWN,
            "icons/chevron_up.svg" => CHEVRON_UP,
            "icons/clear.svg" | "icons/eraser.svg" => CLEAR,
            "icons/copy.svg" => COPY,
            "icons/cut.svg" => CUT,
            "icons/dismiss.svg" | "icons/delete.svg" | "icons/plug_disconnected.svg" => DISMISS,
            "icons/document.svg" | "icons/file.svg" => DOCUMENT,
            "icons/download.svg" | "icons/download_arrow.svg" => DOWNLOAD,
            "icons/upload.svg" | "icons/upload_arrow.svg" => UPLOAD,
            "icons/folder_open.svg" | "icons/folder_sync.svg" => FOLDER,
            "icons/grid.svg" => GRID,
            "icons/light_theme.svg" => LIGHT,
            "icons/dark_theme.svg" => GENERIC,
            "icons/list_tree.svg" => LIST_TREE,
            "icons/minimize.svg" => MINIMIZE,
            "icons/maximize.svg" | "icons/full_screen.svg" | "icons/fit_width.svg" => MAXIMIZE,
            "icons/panel_open.svg" => PANEL_OPEN,
            "icons/paste.svg" | "icons/clipboard_paste.svg" | "icons/clipboard_sync.svg" => PASTE,
            "icons/plug.svg" | "icons/connect.svg" | "icons/network.svg" => PLUG,
            "icons/search.svg" | "icons/find_replace.svg" => SEARCH,
            "icons/settings.svg" => SETTINGS,
            "icons/spinner.svg" => SPINNER,
            path if path.starts_with("icons/") && path.ends_with(".svg") => GENERIC,
            _ => return None,
        })
    }
}

impl AssetSource for FluentAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some(bytes) = Self::builtin(path) {
            return Ok(Some(Cow::Borrowed(bytes)));
        }
        if let Some(fallback) = &self.fallback {
            return fallback.load(path);
        }
        Ok(None)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        if let Some(fallback) = &self.fallback {
            return fallback.list(path);
        }
        let names = if path == "icons" {
            vec![
                "add.svg",
                "arrow_forward.svg",
                "checkmark.svg",
                "chevron_down.svg",
                "chevron_up.svg",
                "clear.svg",
                "copy.svg",
                "dismiss.svg",
                "grid.svg",
                "light_theme.svg",
                "panel_open.svg",
                "plug.svg",
                "search.svg",
                "settings.svg",
                "spinner.svg",
            ]
        } else {
            vec![]
        };
        Ok(names.into_iter().map(SharedString::from).collect())
    }
}
