use fluent_core::{Theme, ThemeProvider as _};
use gpui::{
    div, prelude::*, px, svg, ClickEvent, Context, IntoElement, MouseButton, MouseDownEvent,
    Render, SharedString, Window,
};

/// A custom frameless title bar that replaces the OS-provided one.
///
/// Draggable via `window.start_window_move()`. Renders the app title and
/// window controls (min/max/close) on the right. Enable frameless mode with
/// `TitlebarOptions { appears_transparent: true }` + `WindowDecorations::Client`.
pub struct TitleBar {
    pub title: SharedString,
    pub show_controls: bool,
}

impl TitleBar {
    pub fn new(cx: &mut Context<Self>, title: impl Into<SharedString>) -> Self {
        cx.observe_global::<Theme>(|_, cx| cx.notify()).detach();
        Self {
            title: title.into(),
            show_controls: true,
        }
    }

    pub fn show_controls(mut self, show: bool) -> Self {
        self.show_controls = show;
        self
    }
}

impl Render for TitleBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors.clone();
        let spacing = theme.spacing;
        let typography = theme.typography;

        let title = self.title.clone();
        let show_controls = self.show_controls;

        let bar_bg = colors.surface_dim;
        let fg = colors.on_neutral;
        let ctrl_hover = colors.subtle_hover;
        let close_hover: gpui::Hsla = gpui::rgb(0xC42B1C).into();

        // Drag: left-button press starts window move
        let drag_handler = cx.listener(
            |_: &mut TitleBar, _: &MouseDownEvent, window: &mut Window, _| {
                window.start_window_move();
            },
        );

        let min_handler =
            cx.listener(|_: &mut TitleBar, _: &ClickEvent, window: &mut Window, _| {
                window.minimize_window();
            });
        let max_handler =
            cx.listener(|_: &mut TitleBar, _: &ClickEvent, window: &mut Window, _| {
                window.zoom_window();
            });
        let close_handler =
            cx.listener(|_: &mut TitleBar, _: &ClickEvent, window: &mut Window, _| {
                window.remove_window();
            });

        // The drag area is only the title text, NOT the window control buttons.
        // Putting on_mouse_down on the full bar would intercept clicks on min/max/close.
        let title_area = div()
            .flex_1()
            .h_full()
            .flex()
            .items_center()
            .pl(px(spacing.md))
            .text_size(px(typography.caption.size))
            .text_color(fg)
            .on_mouse_down(MouseButton::Left, drag_handler)
            .child(title);

        let bar = div()
            .flex()
            .flex_row()
            .h(px(36.0))
            .bg(bar_bg)
            .child(title_area);

        if !show_controls {
            return bar;
        }

        // Controls container fills full bar height so hover fills the entire button box.
        bar.child(
            div()
                .flex()
                .flex_row()
                .h_full()
                .child(
                    div()
                        .id("titlebar-min")
                        .w(px(46.0))
                        .h_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .hover(move |s| s.bg(ctrl_hover))
                        .on_click(min_handler)
                        .child(
                            svg()
                                .path("icons/minimize.svg")
                                .size(px(10.0))
                                .text_color(fg),
                        ),
                )
                .child(
                    div()
                        .id("titlebar-max")
                        .w(px(46.0))
                        .h_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .hover(move |s| s.bg(ctrl_hover))
                        .on_click(max_handler)
                        .child(
                            svg()
                                .path("icons/maximize.svg")
                                .size(px(10.0))
                                .text_color(fg),
                        ),
                )
                .child(
                    div()
                        .id("titlebar-close")
                        .w(px(46.0))
                        .h_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .hover(move |s| s.bg(close_hover))
                        .on_click(close_handler)
                        .child(
                            svg()
                                .path("icons/dismiss.svg")
                                .size(px(10.0))
                                .text_color(fg),
                        ),
                ),
        )
    }
}
