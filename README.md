



# FluentGUI

A Rust GUI framework built on [GPUI](https://gpui.rs) with a Fluent 2-inspired design language and a ribbon-centric command model.

[![Crates.io](https://img.shields.io/crates/v/fluent-app)](https://crates.io/crates/fluent-app)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)

---

## Overview

FluentGUI is a reusable, open-source framework that gives Rust applications a coherent Fluent 2 design language on top of [GPUI](https://gpui.rs) — the GPU-accelerated, reactive UI engine from the [Zed](https://zed.dev) editor. It is purpose-built for **desktop applications** that need a polished, professional shell: ribbon bars, resizable dock panels, menus, modals, data surfaces, and full token-driven theming.

**What it is:**
- A complete application shell (title bar, menu bar, ribbon, dock panels, pane splitting)
- A design-token layer mapping Fluent 2 colors, spacing, radii, and type to GPUI primitives
- A library of interactive widgets (inputs, dropdowns, comboboxes, trees, data tables, toasts, modals, and more)
- Live dark/light theme switching with per-entity reactive subscription

**What it is not:**
- A general-purpose layout engine or CSS replacement
- A wrapper around a web renderer
- A port of WinUI/XAML

---

## Demo

<video src="https://github.com/user-attachments/assets/4ba67982-3164-4819-85d8-8c5f964908ec" width="100%" controls></video>

---

## Getting Started

Add the crates you need to your `Cargo.toml`:

```toml
[dependencies]
gpui        = "0.2.2"
fluent-app  = "2.0.2"   # application entry point and window chrome
fluent-core = "2.1.1"   # design tokens and theme (transitive — usually not needed directly)
```

A minimal application:

```rust
use fluent_app::{FluentApp, Workspace};
use gpui::*;

fn main() {
    FluentApp::new("My App")
        .dark_theme()
        .run(|cx| {
            cx.open_window(WindowOptions::default(), |cx| {
                cx.new(|cx| Workspace::new(cx))
            }).unwrap();
        });
}
```

See the [`examples/`](examples/) directory for progressively richer starting points.

---

## Crates

The workspace is split into five independently versioned and published crates. Depend on as many or as few as your application needs.

| Crate | Version | Description |
|-------|---------|-------------|
| [`fluent-core`](crates/fluent-core) | [![](https://img.shields.io/crates/v/fluent-core)](https://crates.io/crates/fluent-core) | Design tokens: semantic colors, spacing, radii, typography, component geometry defaults, and the reactive `Theme` global |
| [`fluent-primitives`](crates/fluent-primitives) | [![](https://img.shields.io/crates/v/fluent-primitives)](https://crates.io/crates/fluent-primitives) | Base interactive widgets — buttons, inputs, dropdowns, comboboxes, checkboxes, avatars, badges, spinners, tooltips, and more |
| [`fluent-ribbon`](crates/fluent-ribbon) | [![](https://img.shields.io/crates/v/fluent-ribbon)](https://crates.io/crates/fluent-ribbon) | `RibbonBar` with tabs, groups, large/compact/toggle/stack buttons, contextual tabs, and overflow handling |
| [`fluent-layout`](crates/fluent-layout) | [![](https://img.shields.io/crates/v/fluent-layout)](https://crates.io/crates/fluent-layout) | Application shell — `Workspace`, `MenuBar`, `ContextMenu`, `DockPanel`, `Pane`, `PaneGroup`, `Tree`, `DataTable`, `Dialog`, `Toast`, and more |
| [`fluent-app`](crates/fluent-app) | [![](https://img.shields.io/crates/v/fluent-app)](https://crates.io/crates/fluent-app) | `FluentApp` builder, frameless title bar, and global initialization of theme, modal, and toast stacks |

### Dependency graph

```
fluent-app
  └── fluent-layout
        └── fluent-ribbon
              └── fluent-primitives
                    └── fluent-core
```

Each layer only depends on layers below it. You can use `fluent-primitives` standalone if you do not need the ribbon or full application shell.

---

## Component Inventory

### fluent-core — Tokens & Theme

- `Theme` global with `apply_dark()`, `apply_light()`, `toggle()` — reactive to all subscribed entities
- `ColorScheme` — full Fluent 2 semantic token set (surface, stroke, fill, accent, status colors)
- `SpacingTokens`, `RadiiTokens`, `TypographyTokens`, `ComponentTokens`
- `MotionTokens` — easing and duration constants for popup and transition animations

### fluent-primitives — Base Widgets

| Widget | Notes |
|--------|-------|
| `Button`, `IconButton`, `ToggleButton` | Accent / Neutral / Subtle / Hyperlink appearances, multiple sizes |
| `TextInput`, `Textarea`, `Searchbox` | Full GPUI IME, clipboard, and focus integration |
| `Field` | Label + description + validation wrapper for any form control |
| `Dropdown`, `Combobox` | Entity-backed, anchored option popover, keyboard navigation |
| `Checkbox`, `Switch`, `RadioGroup` | — |
| `Tooltip` | Hover-triggered; wraps any element |
| `Spinner`, `ProgressBar` | Indeterminate and determinate variants |
| `Avatar`, `Chip`, `Badge`, `Icon`, `Label`, `Divider` | — |

### fluent-ribbon — Ribbon Bar

- `RibbonBar` — tab strip + content row, theme-reactive
- `RibbonTabBuilder` / `RibbonGroupBuilder` — declarative construction
- Item types: `LargeButton`, `Button`, `IconButton`, `ToggleButton`, `Stack`, `Separator`
- `ContextualTabDef` — show/hide tabs based on application state
- Overflow: groups collapse into a *More* dropdown when width is insufficient

### fluent-layout — Shell & Surfaces

**Application shell**
- `Workspace` — assembles title bar, menu bar, ribbon, dock panels, pane area, and modal host
- `MenuBar` — horizontal application menu (File | Edit | View | …) with keyboard navigation and shortcut hints
- `DockPanel` — resizable, collapsible edge panel (left / right / bottom / top)
- `Pane` / `PaneGroup` — content area with optional tab strip; horizontal and vertical splits with draggable dividers
- `TabStrip` / `TabItem`

**Menus & overlays**
- `ContextMenu` — right-click floating menu with icon gutter, shortcuts, separators, checkboxes, radio items, and cascading submenus
- `ModalStack` global + `ModalHost` renderer — click-outside dismiss, multiple `ModalSize` options
- `Dialog` — title / body / actions layout with `ModalStack` integration
- `Popover` — generic floating content surface
- `CommandPalette` — Ctrl+K fuzzy-search command launcher

**Data & feedback**
- `Tree` — hierarchical selection surface with chevrons, icons, disabled rows, callbacks
- `DataTable` — dense tabular surface with sortable columns, selection, loading/empty/error states
- `Toolbar` — compact command strip
- `MessageBar` — inline info / success / warning / error surface with optional dismiss
- `Toast` / `ToastHost` — global notification stack with auto-dismiss and manual dismiss

**Settings UI**
- `SettingsNav` — collapsible section sidebar for settings dialogs with active-item accent bar

### fluent-app — Entry Point

- `FluentApp` builder — configures the window and initializes the `Theme`, `ModalStack`, and `ToastStack` globals
- `TitleBar` — frameless, draggable, with min / max / close controls
- `.dark_theme()` / `.light_theme()` startup options

---

## Examples

| Example | What it shows |
|---------|---------------|
| `hello_world` | Minimal `FluentApp` window — the smallest possible starting point |
| `widgets` | Gallery of all `fluent-primitives` widgets |
| `form` | Form layout with `Field`, validation states, `SettingsNav`, and a `Dialog` |
| `demo_app` | Full application shell: `Workspace`, `MenuBar`, `RibbonBar`, `DockPanel` with a `Tree`, `TabStrip`, `ContextMenu`, `CommandPalette`, and live dark/light theme switching |
| `gallery` | Scrollable overview of all framework components in a single window |

Run any example from the workspace root:

```sh
cargo run -p demo_app
```

---

## Design Principles

1. **Token-driven theming** — every color, spacing value, radius, and type size flows from `fluent-core` tokens; no magic constants in widget code.
2. **Reactive by construction** — entities subscribe to the `Theme` global via `cx.observe_global::<Theme>()` in their constructor; theme changes propagate automatically.
3. **Minimal API surface** — builder patterns with sane defaults; GPUI internals are not exposed unless there is no other option.
4. **Ribbon-first command model** — the `RibbonBar` is the primary command surface; commands are declared and the framework handles layout, overflow, and contextual visibility.
5. **Apache-2.0 throughout** — every public API and implementation is independently authored on Apache-2.0 foundations, safe for use in proprietary applications.

---

## Requirements

| Requirement | Version |
|-------------|---------|
| Rust (MSRV) | **1.88** |
| GPUI | **0.2.2** |
| Platform | Linux (X11 / Wayland), macOS — Windows support follows GPUI |

The GPUI version is deliberately pinned. Upgrades are documented in [docs/STATUS.md](docs/STATUS.md).

### Linux system dependencies

```sh
# Debian / Ubuntu
sudo apt-get install libxkbcommon-dev libxkbcommon-x11-dev
```

---

## Building & Testing

```sh
# Build everything
cargo build --workspace

# Run all tests
cargo test --workspace

# Lint
cargo clippy --workspace --all-targets -- -D warnings

# Format check
cargo fmt --all --check
```

---

## License

Licensed under the [Apache License, Version 2.0](LICENSE).

FluentGUI does not incorporate any GPL-licensed code. The Fluent 2 color token structure was adapted from [aernom/fluent-ui-gpui](https://github.com/aernom/fluent-ui-gpui) (MIT).
