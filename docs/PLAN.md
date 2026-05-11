# FluentGUI — Implementation Plan

## Overview

FluentGUI is a 5-crate Rust framework on top of GPUI providing a ribbon-centric, Fluent 2-inspired application shell. It is built open-source (Apache-2.0) and validated against rdpapp as its primary consumer.

**Phases 0–5 are complete.** The framework is functional and demonstrated by `examples/connect_demo`. Phase 6 is active through the rdpapp GPUI port, with core shell/session flows working and remaining parity work tracked in rdpapp `TODO.md`.

### Crate dependency order (bottom → top)

```
fluent-app
    └── fluent-layout
            └── fluent-ribbon
                    └── fluent-primitives
                            └── fluent-core
                                    └── gpui (0.2.2, Apache-2.0)
```

---

## Phase 0 — Bootstrap ✅ (workspace done; CI/repo push pending)

**Goal**: Compilable workspace, demo apps, repo on Forgejo + GitHub.

### Completed
- Workspace `Cargo.toml` with all 5 crates + 2 examples
- `examples/gallery/` — full framework shell demo
- `examples/connect_demo/` — realistic connection manager dogfood app
- `CLAUDE.md`, `docs/PLAN.md`, `docs/STATUS.md`, `LICENSE` (Apache-2.0)

### Remaining
- `git init` + push to Forgejo `indexarr/fluent-gpui`
- GitHub repo (`AusAgentSmith-org/fluent-gpui`) + push mirror
- `.woodpecker.yml`: `cargo fmt --check` → `cargo clippy -D warnings` → `cargo test --workspace`

---

## Phase 1 — `fluent-core`: Token System & Theme ✅

**Goal**: Single source of truth for all visual properties; reactive theme switching.

### Implemented

**Color Tokens** (`colors.rs`) — `ColorScheme` with dark/light variants:
- Fill backgrounds: neutral, accent, subtle (base/hover/disabled/selected)
- Surface: surface, surface_dim, surface_blur_layer
- Foregrounds: on_neutral, on_accent, on_subtle (+ disabled/selected variants)
- Strokes: stroke_neutral (dim/disabled/subtle), stroke_accent
- Layout additions: panel_bg, panel_border, tab_strip_bg, tab_active_bg, tab_hover_bg
- Ribbon additions: ribbon_bg, ribbon_tab_active_bg, ribbon_tab_indicator, ribbon_group_separator

**Spacing Tokens** (`spacing.rs`) — xs(2) / sm(4) / md(8) / lg(12) / xl(16) / xxl(24)

**Radius Tokens** (`radii.rs`) — sm(2) / md(4) / lg(8) / pill(9999)

**Typography Tokens** (`typography.rs`) — caption(11px) / body(13px) / subtitle(15px) / title(20px) / display(28px)

**Theme Global** (`theme.rs`):
- `Theme` struct — brightness, colors, spacing, radii, typography — implements `Global`
- `ThemeProvider` trait — `cx.theme()` works on any GPUI context via `Deref<Target = App>`
- `Theme::dark()` / `Theme::light()` — construct full themed instances
- `Theme::init(cx)` — register dark theme at startup
- `Theme::apply_dark(cx)` / `Theme::apply_light(cx)` / `Theme::toggle(cx)` — runtime switching
- All `impl Render` entities subscribe via `cx.observe_global::<Theme>(|_, cx| cx.notify()).detach()` in their constructors — full reactive re-render

---

## Phase 2 — `fluent-primitives`: Base Widgets ✅

**Goal**: Atomic building blocks. Stateless display widgets use `RenderOnce`; interactive widgets that need state are entity-backed.

### Implemented

| Widget | File | Notes |
|--------|------|-------|
| `Icon` | `icon.rs` | SVG path, size tokens (16/20/24/32px) |
| `Label` | `label.rs` | Text with TypeRamp token, truncation |
| `Badge` | `badge.rs` | Small pill with count/status |
| `Divider` | `divider.rs` | Horizontal/vertical |
| `Button` | `button.rs` | Appearance (Accent/Neutral/Subtle/Hyperlink), Shape (Rounded/Square/Circular), Size (Normal/Compact) |
| `IconButton` | `icon_button.rs` | Icon-only button variant |
| `ToggleButton` | `toggle_button.rs` | Button with selected state |
| `TextInput` | `text_input.rs` | Editable single-line input; focus, selection, clipboard, IME hooks |
| `Checkbox` | `checkbox.rs` | Three-state (unchecked/checked/indeterminate) |
| `Switch` | `switch.rs` | Toggle switch |
| `Dropdown` | `dropdown.rs` | Entity-backed select with anchored popover and selection callback |
| `Tooltip` | `tooltip.rs` | Hover-triggered GPUI tooltip wrapper |
| `Spinner` | `spinner.rs` | Animated indeterminate loading |
| `ProgressBar` | `progress.rs` | Determinate, 0.0–1.0 |
| `Avatar` | `avatar.rs` | Image or initials fallback |
| `Chip` | `chip.rs` | Dismissable tag |

### API pattern (all widgets)

```rust
Button::new("save")
    .label("Save")
    .appearance(ButtonAppearance::Accent)
    .icon(Icon::new("icons/save.svg"))
    .disabled(false)
    .on_click(|cx| { ... })
```

---

## Phase 3 — `fluent-ribbon`: Ribbon Bar ✅

**Goal**: The framework's flagship component.

### Implemented

**Declarative builder API:**
```rust
cx.new(|cx| {
    RibbonBar::new(cx)
        .tab("Home", |t| {
            t.group("Sessions", |g| {
                g.large_button("Connect", "icons/plug.svg", |_, _, cx| { ... })
                 .button("Disconnect", "icons/close.svg", |_, _, _| {})
                 .stack(|s| {
                     s.button("Clone", "icons/copy.svg", |_, _, _| {})
                      .button("Edit", "icons/edit.svg", |_, _, _| {})
                 })
            })
        })
        .contextual_tab("SSH Session", active_session_is_ssh, |t| {
            t.group("Terminal", |g| {
                g.large_button("Clear", "icons/clear.svg", |_, _, _| {})
            })
        })
})
```

**Component hierarchy:** `RibbonBar` → `RibbonTabDef`[] + `ContextualTabDef`[] → `RibbonGroupDef`[] → `RibbonItemDef`

**Item types:** `LargeButton` (icon above label), `Button` (icon+label side-by-side), `IconButton` (icon only), `ToggleButton`, `Stack` (2–3 stacked compact buttons), `Separator`

**Contextual tabs:** Pass `bool` signal; contextual tab shows/hides with accent band header

Overflow detection and collapse into a compact More dropdown are implemented.

---

## Phase 4 — `fluent-layout`: Pane System, Modals & Menu Bar ✅

**Goal**: Structural shell + overlay system + application menu bar.

### Workspace shell

```
Workspace (relative container)
├── MenuBar (optional, h=28px)
├── RibbonBar (optional)
├── Body row
│   ├── DockPanel left (optional, resizable)
│   ├── Center column
│   │   ├── Pane (content area with optional TabStrip)
│   │   └── DockPanel bottom (optional)
│   └── DockPanel right (optional)
└── ModalHost (absolute overlay, deferred)
```

**Workspace builder API:**
```rust
cx.new(|cx| {
    Workspace::new(cx)
        .menu_bar(menu_bar_entity)
        .ribbon(ribbon_entity.into())
        .left_dock(sidebar_dock)
        .content(pane_entity.into())
        .modal_host(modal_host_entity)
})
```

### Pane / Dock

- **`Pane`** — holds an optional `TabStrip` + content `AnyView`; `set_content()` swaps at runtime
- **`TabStrip`** — horizontal tab bar; `on_select` / `on_close` callbacks; active tab highlight
- **`PaneGroup`** — horizontal/vertical split with ratio API and draggable divider
- **`DockPanel`** — edge panel with 4px drag-handle; `size()` / `collapsed()` / `content()` builder; `collapse_toggle` button

### Modal system

```rust
// Push any modal:
let view = cx.new(|cx| MyModal::new(cx));
ModalStack::push(view.into(), ModalSize::Fixed(600.0, 480.0), cx);

// Pop (from within modal):
ModalStack::pop(cx);

// Generic yes/no:
ConfirmModal::new("Delete connection?", "This cannot be undone.")
    .on_confirm(|cx| { ... })
    .show(cx);
```

`ModalHost` renders via `absolute().inset_0()` within the workspace's `relative()` root — correctly overlays full workspace. The `deferred()` layer ensures it paints above all normal content, and modal stack changes notify the host through GPUI global observers.

### Command Palette

```rust
cx.new(|cx| {
    let mut p = CommandPalette::new(cx);
    p.set_entries(entries, cx);
    p.on_select = Some(Box::new(|entry, cx| { /* handle */ }));
    p
})
```

Open via GPUI action system:
```rust
actions!(my_app, [TogglePalette]);
cx.bind_keys([KeyBinding::new("ctrl-k", TogglePalette, None)]);
// In render:
div().on_action(cx.listener(|_, _: &TogglePalette, _, cx| {
    palette.update(cx, |p, cx| p.show(cx));
}))
```

### Context Menu

```rust
ContextMenu::build(ev.position)
    .action("Connect", |_, _, cx| { ... })
    .action_with_icon("Edit", "icons/edit.svg", |_, _, cx| { ... })
    .separator()
    .action("Delete", |_, _, cx| { ... })
```

Create as an entity (`cx.new`) and include as a child in render. Backdrop closes on click-outside; `cx.stop_propagation()` prevents item clicks from reaching backdrop.

### Settings Nav

```rust
cx.new(move |cx| {
    SettingsNav::new(cx)
        .section(SettingsNavSection::new("Remote Desktop")
            .item(SettingsNavItem::new("general", "Remote Desktop"))
            .item(SettingsNavItem::new("display", "Display Options")))
        .section(SettingsNavSection::new("Common")
            .item(SettingsNavItem::new("network", "Network")))
        .active("general")
})
```

Section headers: body size + semibold. Sub-items: body size + normal, indented to 31px from left (headers at 24px).

### Menu Bar *(added beyond original plan)*

```rust
cx.new(|cx| {
    MenuBar::new(cx)
        .menu("File", vec![
            MenuItemDef::action("New Connection", |_, _, cx| { ... }),
            MenuItemDef::action_with_shortcut("Quick Connect…", "Ctrl+Q", |_, _, cx| { ... }),
            MenuItemDef::separator(),
            MenuItemDef::action("Quit", |_, _, cx| { ... }),
        ])
        .menu("View", vec![
            MenuItemDef::action_with_shortcut("Dark Theme", "Ctrl+Shift+D", |_, _, cx| {
                Theme::apply_dark(cx);
            }),
            MenuItemDef::action_with_shortcut("Light Theme", "Ctrl+Shift+L", |_, _, cx| {
                Theme::apply_light(cx);
            }),
        ])
})
```

Dropdown position is captured from the click event in window coordinates; `anchored()` with `Corner::TopLeft` and `snap_to_window()` positions the panel flush below the trigger. `on_mouse_down_out` closes on click-away.

---

## Phase 5 — `fluent-app`: Application Shell ✅

**Goal**: Entry point that ties everything together.

### FluentApp builder

```rust
FluentApp::new("My App")
    .window_size(1280.0, 800.0)
    .dark_theme()   // or .light_theme()
    .run(|cx| {
        // cx is &mut App
        cx.bind_keys([...]);
        ModalStack::init(cx);
        cx.new(|cx| { /* root entity */ })
    });
```

Initialises `Theme` global, opens GPUI window with `WindowDecorations::Client` (frameless), calls `cx.activate(true)`.

### TitleBar

```rust
cx.new(|cx| TitleBar::new(cx, "My App"))
// or use the helper:
let tb = fluent_app::title_bar("My App", cx);
```

Draggable via `window.start_window_move()`; min/max/close buttons; 28px height; themed.

---

## Phase 6 — rdpapp Port

**Status**: Core GPUI/FluentGUI integration is active in rdpapp.

### Port Strategy

Port screen-by-screen from the old UI, using the framework's modal and pane patterns. Slint is deprecated and is not the target state.

### Component Mapping

| rdpapp Slint file | FluentGUI equivalent |
|-------------------|----------------------|
| `main.slint` | `FluentApp` shell + `RibbonBar` + `MenuBar` |
| `components/sidebar_row.slint` | `ConnectionTreeRow` via `fluent-primitives` |
| `components/common.slint` | `fluent-primitives` widgets |
| All `dialogs/*.slint` (30+) | `ModalStack::push(cx.new(|cx| MyModal::new(cx)).into(), ModalSize::Fixed(...), cx)` |
| `theme.slint` | `fluent-core::Theme` |
| `types.slint` | Moved to rdpapp Rust model layer |

### Session Content Views (rdpapp-specific, not in framework)

Each implements a GPUI element and is placed into a `Pane` via `pane.update(cx, |p, cx| p.set_content(view.into(), cx))`:

- `SshTerminalView` — fontdue + VTE pixel-buffer terminal
- `RdpView` — ironrdp surface
- `VncView` — VNC canvas
- `LocalTermView` — portable-pty + VTE
- `TelnetView`

### Ribbon Layout for rdpapp

```
[Home]  [View]  [Tools]  |  [SSH Connection] (contextual)  [RDP Connection] (contextual)

Home:       New Connection | Sessions (Connect, Disconnect, Clone) | File (Import, Export)
View:       Layout (Grid, List) | Theme (Dark, Light)
Tools:      Key Manager | Known Hosts | Audit Log | Settings
SSH:        Terminal (Clear, Copy, Paste) | SFTP | Port Tunnels
RDP:        Display | Clipboard | Performance
```

### Tasks

- [x] rdpapp starts through `FluentApp`
- [x] Port connection tree sidebar → `DockPanel` + framework `Tree`
- [x] Port session tab strip → `TabStrip` in main `Pane`
- [x] Port ribbon (Home/View + contextual Terminal/Remote Display)
- [x] Implement `SshTermView`, `RdpFrameView`, `VncFrameView`, `TelnetTermView`
- [x] Wire Audit Log / Known Hosts / Settings / Credentials / Search / License modal entry points
- [x] Framework fixes from rdpapp dogfood: content fill hosts, modal overlay sizing, menu/flyout anchoring, submenu placement, focus traversal, ribbon stack wrapping, tree activation
- [ ] Port all 30+ dialogs → modal equivalents (see list in STATUS.md)
- [ ] Re-port full SFTP browser and local terminal surfaces
- [ ] Finish offline/deactivation licensing flows
- [ ] Restore pop-out windows and broader RDP/VNC controls
- [ ] Full test pass, CI green

---

## Cross-Cutting Concerns

### Theme Reactivity Pattern

Every `impl Render` entity that reads `cx.theme()` must subscribe in its constructor:

```rust
pub fn new(cx: &mut Context<Self>) -> Self {
    cx.observe_global::<Theme>(|_, cx| cx.notify()).detach();
    Self { ... }
}
```

`RenderOnce` elements (stateless builders like `Button`, etc.) do not need this — they re-render whenever their parent entity re-renders. Entity-backed widgets such as `Dropdown` and `TextInput` subscribe themselves.

### GPUI Key Binding Pattern

Never use `on_key_down` + `track_focus` for global shortcuts. Always use the action system:

```rust
actions!(my_app, [MyAction]);
cx.bind_keys([KeyBinding::new("ctrl-x", MyAction, None)]);
// In root render:
div().on_action(cx.listener(|this, _: &MyAction, _, cx| { ... }))
```

### Modal Positioning

`ModalHost` must be a child of an element with `.relative()`. The workspace root has `.relative()`. The `ModalHost` renders `div().absolute().inset_0()` which fills the relative container. Do not position `ModalHost` outside the workspace.

### Anchored Dropdown Positioning

`AnchoredPositionMode::Local` inside `deferred()` resolves `bounds.origin` to `(0,0)` — not the trigger's position. Always use `AnchoredPositionMode::Window` (the default) with an explicit `position()` captured from the click event:

```rust
.on_click(move |ev, _, cx| {
    let pos = ev.position();
    let bar_bottom = pos.y + px(BAR_HEIGHT - f32::from(pos.y) % BAR_HEIGHT);
    bar.drop_origin = point(pos.x, bar_bottom);
})
// In render:
anchored().anchor(Corner::TopLeft).position(self.drop_origin).snap_to_window()
```

### GPUI Version Upgrade Path

When upgrading GPUI beyond 0.2.2:
1. Check Zed changelog for element/layout API breaks
2. Update `fluent-core` first (fewest GPUI deps), verify, cascade up
3. Bump all crate minor versions, publish to crates.io
4. Document the GPUI version range in each crate's README
