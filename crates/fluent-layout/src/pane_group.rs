use fluent_core::{Theme, ThemeProvider as _};
use gpui::{
    div, prelude::*, px, AnyView, Context, CursorStyle, DragMoveEvent, IntoElement, MouseButton,
    MouseDownEvent, Render, Window,
};

/// Direction for a pane split.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplitAxis {
    Horizontal,
    Vertical,
}

/// A split layout of two content areas with a configurable, draggable split ratio.
pub struct PaneGroup {
    pub axis: SplitAxis,
    pub first: Option<AnyView>,
    pub second: Option<AnyView>,
    pub ratio: f32,
    drag_start: Option<(f32, f32)>,
    observing_theme: bool,
}

impl Default for PaneGroup {
    fn default() -> Self {
        Self {
            axis: SplitAxis::Horizontal,
            first: None,
            second: None,
            ratio: 0.5,
            drag_start: None,
            observing_theme: false,
        }
    }
}

pub(crate) struct PaneSplitDrag;

impl Render for PaneSplitDrag {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

impl PaneGroup {
    pub fn horizontal() -> Self {
        Self {
            axis: SplitAxis::Horizontal,
            ..Self::default()
        }
    }

    pub fn vertical() -> Self {
        Self {
            axis: SplitAxis::Vertical,
            ..Self::default()
        }
    }

    pub fn first(mut self, view: AnyView) -> Self {
        self.first = Some(view);
        self
    }

    pub fn second(mut self, view: AnyView) -> Self {
        self.second = Some(view);
        self
    }

    pub fn ratio(mut self, ratio: f32) -> Self {
        self.ratio = ratio.clamp(0.1, 0.9);
        self
    }

    pub fn set_first(&mut self, view: AnyView, cx: &mut Context<Self>) {
        self.first = Some(view);
        cx.notify();
    }

    pub fn set_second(&mut self, view: AnyView, cx: &mut Context<Self>) {
        self.second = Some(view);
        cx.notify();
    }
}

impl Render for PaneGroup {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.observing_theme {
            cx.observe_global::<Theme>(|_, cx| cx.notify()).detach();
            self.observing_theme = true;
        }

        let theme = cx.theme();
        let colors = theme.colors.clone();

        let divider_color = colors.stroke_neutral_subtle;
        let divider_hover = colors.stroke_neutral_dim;
        let axis = self.axis;

        let mut container = match self.axis {
            SplitAxis::Horizontal => div().flex().flex_row().flex_1().min_h_0().min_w_0(),
            SplitAxis::Vertical => div().flex().flex_col().flex_1().min_h_0().min_w_0(),
        };

        // First pane
        let first_div = div()
            .min_h_0()
            .min_w_0()
            .overflow_hidden()
            .when(axis == SplitAxis::Horizontal, |d| {
                d.w(gpui::relative(self.ratio)).flex_none()
            })
            .when(axis == SplitAxis::Vertical, |d| {
                d.h(gpui::relative(self.ratio)).flex_none()
            });
        let first_div = if let Some(v) = &self.first {
            first_div.child(v.clone())
        } else {
            first_div
        };
        container = container.child(first_div);

        let start_handler =
            cx.listener(move |group: &mut PaneGroup, ev: &MouseDownEvent, _, _cx| {
                let coord = match axis {
                    SplitAxis::Horizontal => f32::from(ev.position.x),
                    SplitAxis::Vertical => f32::from(ev.position.y),
                };
                group.drag_start = Some((group.ratio, coord));
            });

        let move_handler = cx.listener(
            move |group: &mut PaneGroup, ev: &DragMoveEvent<PaneSplitDrag>, window, cx| {
                if let Some((initial_ratio, start_coord)) = group.drag_start {
                    let bounds = window.bounds();
                    let (current, extent) = match axis {
                        SplitAxis::Horizontal => (
                            f32::from(ev.event.position.x),
                            f32::from(bounds.size.width).max(1.0),
                        ),
                        SplitAxis::Vertical => (
                            f32::from(ev.event.position.y),
                            f32::from(bounds.size.height).max(1.0),
                        ),
                    };
                    let delta = (current - start_coord) / extent;
                    group.ratio = (initial_ratio + delta).clamp(0.1, 0.9);
                    cx.notify();
                }
            },
        );

        // Divider — flex_none prevents it being shrunk to 0 in the flex container
        let divider = match self.axis {
            SplitAxis::Horizontal => div()
                .w(px(5.0))
                .flex_none()
                .cursor(CursorStyle::ResizeLeftRight)
                .bg(divider_color),
            SplitAxis::Vertical => div()
                .h(px(5.0))
                .flex_none()
                .cursor(CursorStyle::ResizeUpDown)
                .bg(divider_color),
        };
        container = container.child(
            divider
                .id("pane-split-divider")
                .hover(move |s| s.bg(divider_hover))
                .on_mouse_down(MouseButton::Left, start_handler)
                .on_drag(PaneSplitDrag, |_, _, _, cx| cx.new(|_| PaneSplitDrag)),
        );

        // Second pane
        let second_div = div().flex_1().min_h_0().min_w_0().overflow_hidden();
        let second_div = if let Some(v) = &self.second {
            second_div.child(v.clone())
        } else {
            second_div
        };
        container
            .on_drag_move::<PaneSplitDrag>(move_handler)
            .child(second_div)
    }
}
