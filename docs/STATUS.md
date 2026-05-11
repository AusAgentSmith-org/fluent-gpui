# FluentGUI — Status

Last updated: 2026-05-10

---

## GPUI Version

Pinned: **0.2.2** (crates.io, Apache-2.0)
Framework release: **v2.0.0**

---

## Phase Summary

| Phase | Crate / Area | Status |
|-------|-------------|--------|
| 0 | Bootstrap — workspace, CI, repos | ✅ Workspace done; CI/repo push pending |
| 1 | `fluent-core` — tokens, theme, component defaults | ✅ Complete |
| 2 | `fluent-primitives` — base/input widgets | ✅ Complete |
| 3 | `fluent-ribbon` — ribbon bar | ✅ Complete |
| 4 | `fluent-layout` — app layout, overlays, data surfaces, menus | ✅ Complete |
| 5 | `fluent-app` — shell entry point | ✅ Complete |
| 6 | rdpapp port | ✅ Core shell/session port active |

---

## Phase 0 — Bootstrap

**Status**: Workspace scaffolded; CI/Forgejo/GitHub push still pending.

- [x] Workspace `Cargo.toml` with all crate members
- [x] `lib.rs` in each crate
- [x] `examples/gallery/` — blank GPUI window with full framework shell
- [x] `examples/connect_demo/` — realistic connection manager demo app
- [x] `CLAUDE.md`, `docs/PLAN.md`, `docs/STATUS.md`
- [x] `LICENSE` (Apache-2.0)
- [ ] `git init` + push to Forgejo `indexarr/fluent-gpui`
- [ ] GitHub repo + mirror push
- [x] `.woodpecker.yml` CI pipeline (fmt → clippy → test)

---

## Phase 1 — `fluent-core`

**Status**: Complete.

- [x] `colors.rs` — `ColorScheme` dark/light (Fluent 2 tokens + ribbon + layout additions)
- [x] `spacing.rs` — `SpacingTokens` (xs/sm/md/lg/xl/xxl)
- [x] `radii.rs` — `RadiiTokens` (sm/md/lg/pill)
- [x] `typography.rs` — `TypeRamp`, `TypographyTokens` (caption/body/subtitle/title/display)
- [x] `theme.rs` — `Theme` global, `ThemeProvider` trait, `Brightness`
- [x] `components.rs` — Fluent/Microsoft-app geometry defaults for menu rows, dropdowns, popups, inputs, dock headers, settings nav, tabs, and ribbon density
- [x] Semantic status colors — info/success/warning/error accents plus matching background and border tokens
- [x] `Theme::apply_dark()`, `Theme::apply_light()`, `Theme::toggle()` — runtime switching
- [x] All `impl Render` entities subscribe via `cx.observe_global::<Theme>()` — full reactive re-render on switch
- [x] `lib.rs` — flat re-exports

---

## Phase 2 — `fluent-primitives`

**Status**: Complete.

Stateless display widgets are `RenderOnce`; interactive widgets that need state are entity-backed:

- [x] `Icon` — SVG path render, size tokens
- [x] `Label` — text with type ramp
- [x] `Divider` + `Badge`
- [x] `Button` / `IconButton` / `ToggleButton` — Appearance (Accent/Neutral/Subtle/Hyperlink), Shape, Size
- [x] `TextInput` — editable single-line input with focus, selection, clipboard, and IME hooks
- [x] `Textarea` — multi-line input with Fluent sizing and focus treatment
- [x] `Field` — label/description/validation wrapper for form controls
- [x] `Checkbox` + `Switch`
- [x] `RadioGroup` — single-selection option list
- [x] `Dropdown` — entity-backed select with anchored option popover and selection callback
- [x] `Combobox` — searchable dropdown with optional freeform query and keyboard highlight
- [x] `Searchbox` — search input with clear/search affordances
- [x] `Tooltip` — hover-triggered GPUI tooltip wrapper
- [x] `Spinner` + `ProgressBar` — animated indeterminate spinner and determinate progress
- [x] `Avatar` + `Chip`

---

## Phase 3 — `fluent-ribbon`

**Status**: Complete.

- [x] `RibbonTabDef` + `RibbonTabBuilder` — declarative tab construction
- [x] `RibbonGroupDef` + `RibbonGroupBuilder` — group with label + separator
- [x] `RibbonItemDef` — `LargeButton`, `Button`, `IconButton`, `ToggleButton`, `Stack`, `Separator`
- [x] `RibbonStackBuilder` — 2–3 stacked compact buttons per column
- [x] `ContextualTabDef` — show/hide tabs based on app state (`bool` signal)
- [x] `RibbonBar` — assembles tabs + groups; manages active tab; contextual tab visibility
- [x] Tab click navigation; contextual tab accent band
- [x] Theme-reactive: subscribes to `Theme` global in constructor
- [x] Overflow detection + collapse into a compact More dropdown when groups exceed available width

---

## Phase 4 — `fluent-layout`

**Status**: Complete. Includes `MenuBar`, popup/data surfaces, and notification primitives beyond the original scope.

### Pane / Dock System
- [x] `TabItem` + `TabStrip` — active state, close handler
- [x] `Pane` — content area with optional tab strip
- [x] `PaneGroup` — horizontal/vertical split with ratio API and draggable divider
- [x] `DockPanel` — resizable edge panel (left/right/bottom/top), collapse toggle, drag-to-resize
- [x] `Workspace` — assembles menu bar + ribbon + dock panels + pane area + modal host
- [x] All pane/dock entities theme-reactive

### Modal / Overlay System
- [x] `ModalStack` — GPUI global modal stack
- [x] `ModalHost` — deferred overlay renderer; observes modal stack changes and supports click-outside dismiss
- [x] `ModalSize` — Fit / Fixed / Fraction
- [x] `ConfirmModal` — generic yes/no modal with confirm/cancel callbacks and stack pop
- [x] `AddConnectionModal` in demo — sectioned form with `SettingsNav` sidebar (30+ dialog pattern established)

### Command Palette
- [x] `CommandPalette` — Ctrl+K floating search; fuzzy filter on label/subtitle/keywords
- [x] `PaletteEntry` — id, label, subtitle, icon, keywords
- [x] Self-focuses on open; Escape closes; Enter confirms; ↑↓ navigate
- [x] `on_select` callback
- [x] GPUI `actions!` + `bind_keys` wiring (not focus-dependent)

### Context Menu
- [x] `ContextMenu` — right-click floating menu; `anchored()` + `deferred()` positioning
- [x] `ContextMenuItem` — Action, shortcut, icon, disabled, checkbox, radio, separator, submenu/cascade rows
- [x] Click-outside backdrop dismiss; `cx.stop_propagation()` on item click
- [x] Fluent-like icon gutter, menu row height, separator height, minimum width, and submenu placement from `ComponentTokens`
- [x] Recursive submenu placement plus keyboard roving selection (Up/Down/Left/Right/Escape/Enter/Space)

### Settings Nav
- [x] `SettingsNav` — left sidebar nav for settings/data-entry modals
- [x] `SettingsNavSection` — collapsible sections; ▼/▶ chevron (properly vertically centred)
- [x] `SettingsNavItem` — active state with 3px accent bar; correct indentation hierarchy
- [x] Section headers larger than sub-items (body+semibold vs body+normal)

### Menu Bar *(added beyond original plan)*
- [x] `MenuBar` — horizontal application menu bar (File | Edit | View | …)
- [x] `MenuItemDef` — Action / Action+Shortcut / Action+Icon / Separator
- [x] Click trigger opens dropdown; click-away closes via `on_mouse_down_out`
- [x] Window-coordinate positioning via captured click position (below trigger, not overlapping)
- [x] Shortcut hints right-aligned in dropdown rows
- [x] No hover-switch between menus (user preference)
- [x] Keyboard roving selection and activation (Left/Right/Up/Down/Escape/Enter/Space)
- [x] Wired to `Workspace` as `.menu_bar()` above ribbon

### Toolbar / Data / Feedback Surfaces
- [x] `Toolbar` — compact command strip with buttons, toggles, dropdown-capable rows, and separators
- [x] `Tree` — hierarchical selection surface with chevrons, icons, disabled rows, and toggle/select callbacks
- [x] `DataTable` — dense tabular surface with structured cells, alignment, selection, sortable columns, text/number sorting, loading/empty/error states
- [x] `Tree` / `DataTable` controlled keyboard movement callbacks for app-owned selection state
- [x] `Popover` — floating content surface with Fluent border/background defaults
- [x] `Dialog` — title/body/actions layout with default dismiss button and `ModalStack` integration
- [x] `MessageBar` — inline semantic info/success/warning/error surface with optional dismiss
- [x] `Toast` / `ToastHost` — global notification stack with default placement, width, max-entry policy, auto-dismiss timeout, and manual dismiss

---

## Phase 5 — `fluent-app`

**Status**: Complete.

- [x] `TitleBar` — frameless, draggable, window controls (min/max/close)
- [x] `FluentApp` builder — initialises theme global, sets window size, opens GPUI window
- [x] Initialises `ModalStack` and `ToastStack` globals for overlays/notifications
- [x] `title_bar()` factory helper
- [x] `.dark_theme()` / `.light_theme()` startup options
- [x] Theme-reactive: TitleBar subscribes to `Theme` global

---

## Phase 6 — rdpapp Port

**Status**: Core GPUI/FluentGUI shell is active in rdpapp.

- [x] rdpapp starts through `FluentApp`
- [x] `Workspace` hosts title bar, menu bar, ribbon, left dock, pane, status bar, and modal host
- [x] Sidebar uses FluentGUI `Tree` with selection, activation, and context menu flows
- [x] Home/View/Terminal/Remote Display ribbon wiring is active
- [x] SSH, RDP, VNC, and Telnet session views render inside `Pane`
- [x] Audit Log, Known Hosts, Settings, Credentials, Search, and License modals are reachable
- [x] Framework fixes driven by rdpapp: full-height content hosting, modal overlay fill, menu/flyout anchoring, submenu placement, ribbon stack wrapping, focus traversal, tree activation
- [ ] Remaining rdpapp parity: full SFTP browser, richer credential CRUD, offline/deactivation licensing flows, pop-out windows, import/export UI polish

---

## connect_demo — Demo App

**Status**: Feature-complete for framework demonstration purposes.

Demonstrates the full framework shell:

- `MenuBar` (File / View / Tools / Help) with live Dark/Light Theme switching
- `RibbonBar` with Home, View, Tools tabs; contextual SSH/RDP tabs when a session is active
- Left `DockPanel` sidebar with `NavTree` (groups + servers, right-click context menu)
- `TabStrip` + `Pane` for open sessions (Dashboard + SSH/RDP/VNC session views)
- `AddConnectionModal` — multi-section data-entry modal with `SettingsNav`
- `CommandPalette` (Ctrl+K) populated with all servers
- `ConfirmModal` for disconnect confirmation
- Live theme switching: View menu → Dark/Light Theme, or Ctrl+Shift+D / Ctrl+Shift+L

Known issues / not yet implemented in demo:
- Quick Connect / import / export / help actions are still app-level placeholders
- SFTP, tunnel, credential dialogs not implemented (framework modal pattern is established)

---

## Known Issues / Deferred

| Issue | Severity | Notes |
|-------|----------|-------|
| Demo action wiring | Low | A few demo menu items remain placeholders; framework patterns are in place |

---

## Decisions Log

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-05-08 | GPUI 0.2.2 pinned from crates.io | Stable, Apache-2.0, avoids git dep complexity |
| 2026-05-08 | Ribbon scope: simplified (no QAT, no keytips, no mini toolbar) | Scope control; covers 80% of use cases |
| 2026-05-08 | Pane/dock system included in framework | Required by rdpapp; generic enough to be reusable |
| 2026-05-08 | Adopt aernom's ColorScheme structure (MIT) | Good Fluent 2 mapping; saves token design work |
| 2026-05-08 | rdpapp is the primary dogfood consumer | Drives real requirements; prevents over-abstraction |
| 2026-05-08 | Apache-2.0 licence | Maximally reusable, including in proprietary apps |
| 2026-05-08 | MenuBar added to framework | Real apps need both ribbon and classic menu bar; Workspace gains `.menu_bar()` slot |
| 2026-05-08 | Theme reactivity via `cx.observe_global::<Theme>()` per entity | GPUI's correct pattern; each entity subscribes independently in its constructor |
| 2026-05-08 | Menu hover-switch removed | User preference: open on click only, not hover |
| 2026-05-08 | Dropdown anchor uses captured click position (Window mode) | `AnchoredPositionMode::Local` inside `deferred()` resolves to (0,0), not trigger position |
| 2026-05-10 | Component geometry defaults are framework tokens | Fluent-like row heights, popup widths, icon gutters, focus indicators, and overlay sizing are framework concerns, not app-by-app styling chores |
| 2026-05-10 | Add broad primitive/layout coverage before release | The library is unreleased, so API-breaking cleanup is acceptable while aligning with Fluent 2 and Microsoft app defaults |
