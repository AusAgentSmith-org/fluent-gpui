use std::sync::Arc;

use fluent_core::Theme;
use fluent_layout::{modal::ModalStack, ToastStack};
use gpui::{
    prelude::*, px, size, App, Application, AssetSource, Bounds, Entity, Render, SharedString,
    TitlebarOptions, WindowBounds, WindowDecorations, WindowOptions,
};

use crate::{assets::FluentAssets, title_bar::TitleBar};

const DEFAULT_W: f32 = 1280.0;
const DEFAULT_H: f32 = 800.0;

/// Builder for a FluentGUI application.
///
/// ```ignore
/// FluentApp::new("MyApp")
///     .window_size(1440.0, 900.0)
///     .run(|cx| {
///         cx.new(|_| Workspace::new()...)
///     });
/// ```
pub struct FluentApp {
    title: SharedString,
    window_w: f32,
    window_h: f32,
    dark: bool,
    assets: Option<Arc<dyn AssetSource>>,
}

impl FluentApp {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            window_w: DEFAULT_W,
            window_h: DEFAULT_H,
            dark: true,
            assets: None,
        }
    }

    pub fn window_size(mut self, w: f32, h: f32) -> Self {
        self.window_w = w;
        self.window_h = h;
        self
    }

    pub fn dark_theme(mut self) -> Self {
        self.dark = true;
        self
    }

    pub fn light_theme(mut self) -> Self {
        self.dark = false;
        self
    }

    pub fn assets(mut self, assets: impl AssetSource) -> Self {
        self.assets = Some(Arc::new(assets));
        self
    }

    /// Launch the application.
    ///
    /// The `build` closure runs on the main thread with `&mut App` and must
    /// return the root `Entity<V>` to display as the window's content.
    pub fn run<V: Render + 'static>(self, build: impl FnOnce(&mut App) -> Entity<V> + 'static) {
        let title = self.title.clone();
        let w = self.window_w;
        let h = self.window_h;
        let dark = self.dark;
        let assets = self.assets;

        Application::new()
            .with_assets(FluentAssets::new(assets))
            .run(move |cx: &mut App| {
                if dark {
                    Theme::init(cx);
                } else {
                    cx.set_global(Theme::light());
                }
                ModalStack::init(cx);
                ToastStack::init(cx);

                let bounds = Bounds::centered(None, size(px(w), px(h)), cx);

                cx.open_window(
                    WindowOptions {
                        window_bounds: Some(WindowBounds::Windowed(bounds)),
                        titlebar: Some(TitlebarOptions {
                            title: Some(title.clone()),
                            appears_transparent: true,
                            traffic_light_position: None,
                        }),
                        window_decorations: Some(WindowDecorations::Client),
                        ..Default::default()
                    },
                    move |_window, cx: &mut App| build(cx),
                )
                .unwrap();

                cx.activate(true);
            });
    }
}

/// Create a `TitleBar` entity — include as the first child of your `Workspace`.
pub fn title_bar(title: impl Into<SharedString>, cx: &mut App) -> Entity<TitleBar> {
    let t = title.into();
    cx.new(|cx| TitleBar::new(cx, t))
}
