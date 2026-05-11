use fluent_core::{Theme, ThemeProvider as _};
use gpui::{
    deferred, div, prelude::*, px, App, Context, FocusHandle, FontWeight, IntoElement,
    KeyDownEvent, Render, SharedString, Window,
};

// ---------------------------------------------------------------------------
// Entry type
// ---------------------------------------------------------------------------

/// A single searchable entry in the `CommandPalette`.
#[derive(Clone, Debug)]
pub struct PaletteEntry {
    pub id: SharedString,
    /// Primary display text — also used in filtering.
    pub label: SharedString,
    /// Secondary line (e.g. host/path/shortcut).
    pub subtitle: Option<SharedString>,
    /// Optional icon SVG path.
    pub icon: Option<SharedString>,
    /// Extra keywords included in the fuzzy match but not displayed.
    pub keywords: Vec<SharedString>,
}

impl PaletteEntry {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            subtitle: None,
            icon: None,
            keywords: vec![],
        }
    }

    pub fn subtitle(mut self, s: impl Into<SharedString>) -> Self {
        self.subtitle = Some(s.into());
        self
    }

    pub fn icon(mut self, path: impl Into<SharedString>) -> Self {
        self.icon = Some(path.into());
        self
    }

    pub fn keyword(mut self, kw: impl Into<SharedString>) -> Self {
        self.keywords.push(kw.into());
        self
    }
}

// ---------------------------------------------------------------------------
// CommandPalette entity
// ---------------------------------------------------------------------------

/// A Ctrl+K style floating search / action palette.
///
/// Include one instance as a child of your root view. Call `.show()` (or set
/// `open = true` + `cx.notify()`) to reveal it. It self-focuses on the next
/// frame and handles all keyboard input internally.
///
/// Results filter in real time as the user types (case-insensitive substring
/// match against `label`, `subtitle`, and `keywords`). The `on_select`
/// callback fires when the user confirms a result.
#[allow(clippy::type_complexity)]
pub struct CommandPalette {
    pub open: bool,
    focus_handle: FocusHandle,
    query: String,
    entries: Vec<PaletteEntry>,
    filtered: Vec<usize>,
    selected: usize,
    pub on_select: Option<Box<dyn Fn(&PaletteEntry, &mut App) + 'static>>,
}

impl CommandPalette {
    pub fn new(cx: &mut Context<Self>) -> Self {
        cx.observe_global::<Theme>(|_, cx| cx.notify()).detach();
        Self {
            open: false,
            focus_handle: cx.focus_handle(),
            query: String::new(),
            entries: vec![],
            filtered: vec![],
            selected: 0,
            on_select: None,
        }
    }

    // ---- Public API ----

    pub fn entries(mut self, entries: Vec<PaletteEntry>) -> Self {
        self.entries = entries;
        self.refilter();
        self
    }

    pub fn set_entries(&mut self, entries: Vec<PaletteEntry>, cx: &mut Context<Self>) {
        self.entries = entries;
        self.refilter();
        cx.notify();
    }

    pub fn on_select(mut self, f: impl Fn(&PaletteEntry, &mut App) + 'static) -> Self {
        self.on_select = Some(Box::new(f));
        self
    }

    pub fn show(&mut self, cx: &mut Context<Self>) {
        self.open = true;
        self.query.clear();
        self.selected = 0;
        self.refilter();
        cx.notify();
    }

    pub fn hide(&mut self, cx: &mut Context<Self>) {
        self.open = false;
        cx.notify();
    }

    // ---- Internal ----

    fn refilter(&mut self) {
        let q = self.query.to_lowercase();
        if q.is_empty() {
            self.filtered = (0..self.entries.len()).collect();
        } else {
            self.filtered = self
                .entries
                .iter()
                .enumerate()
                .filter(|(_, e)| {
                    e.label.to_lowercase().contains(&q)
                        || e.subtitle
                            .as_ref()
                            .map(|s| s.to_lowercase().contains(&q))
                            .unwrap_or(false)
                        || e.keywords.iter().any(|kw| kw.to_lowercase().contains(&q))
                })
                .map(|(i, _)| i)
                .collect();
        }
        self.selected = self.selected.min(self.filtered.len().saturating_sub(1));
    }

    fn move_up(&mut self, cx: &mut Context<Self>) {
        if !self.filtered.is_empty() && self.selected > 0 {
            self.selected -= 1;
        }
        cx.notify();
    }

    fn move_down(&mut self, cx: &mut Context<Self>) {
        if self.selected + 1 < self.filtered.len() {
            self.selected += 1;
        }
        cx.notify();
    }

    fn confirm(&mut self, cx: &mut Context<Self>) {
        if let Some(&entry_idx) = self.filtered.get(self.selected) {
            if let Some(entry) = self.entries.get(entry_idx) {
                let entry = entry.clone();
                if let Some(f) = &self.on_select {
                    f(&entry, cx);
                }
            }
        }
        self.hide(cx);
    }

    fn handle_key(&mut self, ev: &KeyDownEvent, cx: &mut Context<Self>) {
        match ev.keystroke.key.as_str() {
            "escape" => self.hide(cx),
            "return" => self.confirm(cx),
            "up" => self.move_up(cx),
            "down" => self.move_down(cx),
            "backspace" => {
                self.query.pop();
                self.refilter();
                cx.notify();
            }
            _ => {
                // Printable character: use key_char (includes shifted characters)
                if let Some(ch) = &ev.keystroke.key_char {
                    if !ev.keystroke.modifiers.control && !ev.keystroke.modifiers.platform {
                        self.query.push_str(ch);
                        self.refilter();
                        cx.notify();
                    }
                }
            }
        }
    }
}

impl Render for CommandPalette {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.open {
            return div();
        }

        // Self-focus on the first frame after opening
        if !self.focus_handle.is_focused(window) {
            window.focus(&self.focus_handle);
        }

        let theme = cx.theme();
        let colors = theme.colors.clone();
        let spacing = theme.spacing;
        let typography = theme.typography;
        let radii = theme.radii;
        let components = theme.components;

        let query_display = if self.query.is_empty() {
            "Type to search…".to_string()
        } else {
            format!("{}_", self.query) // underscore acts as cursor indicator
        };

        let input_text_color = if self.query.is_empty() {
            colors.on_subtle_disabled
        } else {
            colors.on_neutral
        };

        let selected = self.selected;
        let filtered = self.filtered.clone();
        let entries = &self.entries;

        // Build result rows
        let mut results = div().flex().flex_col().max_h(px(360.0)).overflow_hidden();

        for (row_idx, &entry_idx) in filtered.iter().enumerate() {
            let Some(entry) = entries.get(entry_idx) else {
                continue;
            };
            let is_selected = row_idx == selected;
            let label = entry.label.clone();
            let subtitle = entry.subtitle.clone();
            let icon = entry.icon.clone();

            let row_bg = if is_selected {
                colors.neutral_selected
            } else {
                colors.surface
            };
            let row_hover = colors.neutral_hover;
            let label_fg = colors.on_neutral;
            let sub_fg = colors.on_subtle;

            let row = cx.listener(move |p: &mut CommandPalette, _, _, cx| {
                p.selected = row_idx;
                p.confirm(cx);
            });

            let mut row_el = div()
                .id(("palette-row", row_idx as u64))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(spacing.sm))
                .px(px(spacing.md))
                .py(px(spacing.sm))
                .bg(row_bg)
                .cursor_pointer()
                .hover(move |s| s.bg(row_hover))
                .on_click(row);

            if let Some(icon_path) = icon {
                row_el = row_el.child(
                    gpui::svg()
                        .path(icon_path)
                        .size(px(components.command_palette_icon_slot))
                        .text_color(colors.on_neutral),
                );
            } else {
                row_el = row_el.child(div().size(px(components.command_palette_icon_slot)));
            }

            let mut text_col = div().flex().flex_col().flex_1().child(
                div()
                    .text_size(px(typography.body.size))
                    .text_color(label_fg)
                    .when(is_selected, |d| d.font_weight(FontWeight::SEMIBOLD))
                    .child(label),
            );

            if let Some(sub) = subtitle {
                text_col = text_col.child(
                    div()
                        .text_size(px(typography.caption.size))
                        .text_color(sub_fg)
                        .child(sub),
                );
            }

            row_el = row_el.child(text_col);
            results = results.child(row_el);
        }

        // Empty state
        if filtered.is_empty() {
            results = results.child(
                div()
                    .px(px(spacing.md))
                    .py(px(spacing.lg))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(typography.body.size))
                    .text_color(colors.on_subtle_disabled)
                    .child("No results"),
            );
        }

        // Main palette panel
        let panel = div()
            .w(px(560.0))
            .bg(colors.surface)
            .border_1()
            .border_color(colors.stroke_neutral)
            .rounded(px(radii.lg))
            .shadow_lg()
            .overflow_hidden()
            // Search input row
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(spacing.sm))
                    .px(px(spacing.md))
                    .h(px(components.command_palette_search_height))
                    .border_b_1()
                    .border_color(colors.stroke_neutral_subtle)
                    .child(
                        gpui::svg()
                            .path("icons/search.svg")
                            .size(px(components.command_palette_icon_slot))
                            .text_color(colors.on_subtle),
                    )
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(typography.body.size))
                            .text_color(input_text_color)
                            .child(query_display),
                    ),
            )
            .child(results)
            // Footer hint
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .gap(px(spacing.md))
                    .px(px(spacing.md))
                    .h(px(components.command_palette_footer_height))
                    .border_t_1()
                    .border_color(colors.stroke_neutral_subtle)
                    .child(
                        div()
                            .text_size(px(typography.caption.size))
                            .text_color(colors.on_subtle_disabled)
                            .child("↑↓ navigate · ↵ select · Esc close"),
                    ),
            );

        // Backdrop + centered panel, all deferred so it floats above everything
        div().child(deferred(
            div()
                .absolute()
                .inset_0()
                .flex()
                .items_center()
                .justify_center()
                .pt(px(80.0)) // Appear in the upper-centre (like VS Code)
                .items_start()
                // Backdrop closes on click
                .child(
                    div()
                        .id("palette-backdrop")
                        .absolute()
                        .inset_0()
                        .on_mouse_down(
                            gpui::MouseButton::Left,
                            cx.listener(|p: &mut CommandPalette, _, _, cx| p.hide(cx)),
                        ),
                )
                .child(
                    // Panel container: relative so it's above the backdrop
                    div()
                        .id("palette-panel")
                        .relative()
                        .track_focus(&self.focus_handle)
                        .on_key_down(cx.listener(
                            |p: &mut CommandPalette, ev: &KeyDownEvent, _, cx| {
                                p.handle_key(ev, cx);
                            },
                        ))
                        .child(panel),
                ),
        ))
    }
}
