use fluent_core::ThemeProvider as _;
use gpui::{
    div, img, prelude::*, px, App, IntoElement, ObjectFit, RenderOnce, SharedString, Window,
};

/// A circular avatar: initials fallback or image.
#[derive(IntoElement)]
pub struct Avatar {
    initials: Option<SharedString>,
    image_path: Option<SharedString>,
    size: f32,
}

impl Avatar {
    pub fn initials(initials: impl Into<SharedString>) -> Self {
        Self {
            initials: Some(initials.into()),
            image_path: None,
            size: 32.0,
        }
    }

    pub fn image(path: impl Into<SharedString>) -> Self {
        Self {
            initials: None,
            image_path: Some(path.into()),
            size: 32.0,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl RenderOnce for Avatar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let colors = &theme.colors;
        let typography = &theme.typography;

        let base = div()
            .size(px(self.size))
            .rounded(px(9999.0))
            .flex()
            .items_center()
            .justify_center()
            .overflow_hidden();

        if let Some(initials) = self.initials {
            base.bg(colors.accent)
                .text_color(colors.on_accent)
                .text_size(px(typography.caption.size))
                .child(initials)
        } else if let Some(path) = self.image_path {
            base.child(img(path).size_full().object_fit(ObjectFit::Cover))
        } else {
            base.bg(colors.neutral)
        }
    }
}
