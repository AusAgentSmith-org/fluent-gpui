use fluent_core::{ThemeProvider as _, TypographyTokens};
use gpui::{
    div, prelude::*, px, App, FontWeight, Hsla, IntoElement, RenderOnce, SharedString, Window,
};

/// Which step of the type ramp to use.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LabelSize {
    Caption,
    #[default]
    Body,
    Subtitle,
    Title,
    Display,
}

/// A text label styled with the active theme's type ramp.
///
/// ```ignore
/// Label::new("Hello").size(LabelSize::Subtitle).color(cx.theme().colors.on_accent)
/// ```
#[derive(IntoElement)]
pub struct Label {
    text: SharedString,
    size: LabelSize,
    color: Option<Hsla>,
    truncate: bool,
}

impl Label {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            size: LabelSize::default(),
            color: None,
            truncate: false,
        }
    }

    pub fn size(mut self, size: LabelSize) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Clip text with an ellipsis when it overflows.
    pub fn truncate(mut self) -> Self {
        self.truncate = true;
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let color = self.color.unwrap_or(theme.colors.on_neutral);
        let t = &theme.typography;
        let ramp = ramp_for_size(self.size, t);
        let weight = FontWeight(ramp.weight as f32);

        let el = div()
            .text_size(px(ramp.size))
            .line_height(px(ramp.line_height))
            .font_weight(weight)
            .text_color(color)
            .child(self.text);

        if self.truncate {
            el.truncate().whitespace_nowrap()
        } else {
            el
        }
    }
}

fn ramp_for_size(size: LabelSize, t: &TypographyTokens) -> fluent_core::TypeRamp {
    match size {
        LabelSize::Caption => t.caption,
        LabelSize::Body => t.body,
        LabelSize::Subtitle => t.subtitle,
        LabelSize::Title => t.title,
        LabelSize::Display => t.display,
    }
}
