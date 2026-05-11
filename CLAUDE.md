# FluentGUI — Claude Context

FluentGUI is a reusable, open-source Rust GUI framework built on [GPUI](https://gpui.rs) (Zed's Apache-2.0 GPU-accelerated UI engine). Its defining characteristic is a **ribbon-centric command model** with a Fluent 2-inspired design language. The primary consumer driving development is **rdpapp** (a native SSH/RDP/VNC connection manager at `~/Working/Active/rdpapp`).

---

## Crate Workspace

```
fluent-gpui/                     ← workspace root (open-sourced on GitHub)
├── crates/
│   ├── fluent-core/             ← design tokens, theme, color scheme (no GPUI dep)
│   ├── fluent-primitives/       ← base widgets: Button, Input, Label, Badge, Icon, etc.
│   ├── fluent-ribbon/           ← RibbonBar, RibbonTab, RibbonGroup, contextual tabs
│   ├── fluent-layout/           ← Pane/dock splitting, modal stack, workspace shell
│   └── fluent-app/              ← App entry point, window chrome, custom title bar
├── examples/
│   └── gallery/                 ← dogfood app demonstrating the full framework
└── docs/
    ├── PLAN.md                  ← detailed implementation plan
    └── STATUS.md                ← phase-by-phase status tracker
```

Each crate is independently versioned and published to **crates.io** (Apache-2.0).

---

## GPUI Version

Pinned to **`gpui = "0.2.2"`** from crates.io. Do not use path or git dependencies for GPUI — the published crate is Apache-2.0 and stable. Upgrades are deliberate and documented in STATUS.md.

---

## License

**Apache 2.0** throughout. This is non-negotiable — the whole point of the framework is to be reusable in proprietary apps.

- Do NOT copy or adapt code from Zed's GPL-licensed crates (`crates/ui`, `crates/workspace`, `crates/dock`, `crates/pane`, `crates/title_bar`). Read them for design inspiration only.
- aernom's [fluent-ui-gpui](https://github.com/aernom/fluent-ui-gpui) is MIT — patterns (theme global, color scheme, button enums) may be adapted freely.

---

## Design Principles

1. **Ribbon-first**: The `RibbonBar` is the primary command surface. Apps describe their commands declaratively; the framework handles layout, overflow, and contextual visibility.
2. **Token-driven theming**: All visual properties (color, spacing, radius, type) flow from `fluent-core` tokens. No magic constants in widget code.
3. **Minimal API surface**: Prefer builder patterns with sane defaults. Don't expose GPUI internals unless necessary.
4. **No GPL contamination**: Every public API and implementation must be independently authored on Apache-2.0 foundations.
5. **rdpapp is the dogfood app**: Design decisions must be validated against rdpapp's actual requirements before being treated as settled.

---

## Reference Material

| Resource | Purpose |
|----------|---------|
| `~/Working/Active/apps/zed/crates/gpui/` | GPUI source — authoritative for element API, layout, globals |
| `~/Working/Active/apps/zed/crates/ui/` | Zed's GPL widget layer — read for design ideas, never copy |
| `~/Working/Active/apps/zed/crates/workspace/` | Zed's GPL pane/dock model — read for architecture ideas only |
| `~/Working/Active/rdpapp/` | Primary consumer app — drives all feature requirements |
| https://github.com/aernom/fluent-ui-gpui | Reference for theme/button patterns (MIT) |
| https://fluent2.microsoft.design | Fluent 2 design specification |

---

## Build & Test

```bash
# From workspace root
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
```

There are no platform-specific build steps at the framework level. The `examples/gallery` app is the primary integration test surface.

---

## Source Control & Publishing

- **Forgejo** (`repo.indexarr.net/indexarr/fluent-gpui`) — source of truth, Woodpecker CI
- **GitHub** (`github.com/AusAgentSmith-org/fluent-gpui` or a dedicated org) — public mirror, the open-source face
- Push Forgejo first, GitHub second (per workspace CLAUDE.md convention)
- Crates publish to **crates.io** (not the Forgejo cargo registry — this is public)

---

## Known Invariants & Pitfalls (learned from rdpapp)

### Layout

- **Flex handles must have `flex_none()`**: Any resize handle or fixed-size divider in a flex container needs `.flex_none()` or flex layout will shrink it to 0px when a sibling has `flex_1()`. This applies to `DockPanel`'s resize handle and `PaneGroup`'s split divider.
- **`h_full()` requires `align-items: stretch` on parent**: If the parent flex container has `.items_center()`, children sized with `.h_full()` reference the parent's *content* height, not the flex container height. Either omit `.items_center()` on the parent (default is stretch) and centre content explicitly, or give the child `.self_stretch()`.
- **`overflow_y_scroll()` requires `StatefulInteractiveElement`**: Only elements with an `.id(...)` implement `StatefulInteractiveElement` and therefore expose `overflow_y_scroll()`. Always set an id before calling this method.

### Event Handling

- **Title bar drag vs. buttons**: Placing `on_mouse_down(start_window_move)` on the full title bar div intercepts clicks on child buttons (min/max/close) — the window-move gesture starts before the click completes. Attach the drag handler only to the title *text area* element, not the bar container.
- **`on_action` needs `track_focus`**: A div's `on_action` handler only fires when that div is in the GPUI focus responder chain. Root views that handle global actions (e.g., Ctrl+K) must hold a `FocusHandle`, call `track_focus(&handle)` on their rendered div, and focus the handle when no child is focused.

### Positioning

- **`MenuBar` dropdown anchoring**: The dropdown uses `anchored().position(origin)` with window-absolute coordinates. Do NOT compute the bar's Y position from `click_y % BAR_HEIGHT` — this is wrong when the bar isn't at y=0. Instead, measure the bar's actual Y position using a 0×0 `canvas` child whose prepaint closure writes to an `Arc<Mutex<f32>>`. Read the stored value in click handlers.
- **`canvas` closures are `FnOnce`**: Each render creates a fresh canvas element with a new `FnOnce` closure. State updates from prepaint must use `Arc<Mutex<T>>` (not `WeakEntity`) since entity mutation during rendering causes borrow conflicts.

---

## What NOT to Do

- Do not add animation or motion beyond hover/focus state transitions — out of scope for MVP
- Do not implement Mica/Acrylic material effects — they require platform APIs not exposed by GPUI 0.2
- Do not model the pane/dock system on Zed's text-buffer-centric assumptions — our content is arbitrary
- Do not skip the token layer and hardcode colors/sizes in widget implementations
- Do not add `Co-Authored-By` Claude lines to commits (per workspace policy)
