# FluentGUI — Visual Target Reference

The screenshots in `ExternalExamples/` are from **Royal TS** (a commercial SSH/RDP/VNC
connection manager). Royal TS is the closest public equivalent to rdpapp and serves as the
definitive visual reference for every FluentGUI component.

---

## Screenshots

| File | Shows |
|------|-------|
| `Generic Ribbon.png` | Full app shell: title bar, ribbon (Home tab active), navigation sidebar, content tab strip, dual-pane content area |
| `Ribbon Tree.png` | Same shell zoomed into the sidebar tree + ribbon at ~2× — fine detail of icons, indentation, separators |
| `Ribbon Dropdown.png` | "Add" large-button dropdown open — flat menu anchored below the ribbon button |
| `File Explorer Basic.png` | Dual-pane file explorer layout (two `Pane`s side-by-side, each with its own list view) |
| `Generic Data Entry.png` | Add-connection dialog: left collapsible nav tree + right form with labeled fields, underline focus style |
| `Right Click Context Menu.png` | Right-click context menu on a tree item with a cascading submenu |

---

## Component-by-Component Observations

### Title Bar (`fluent-app::TitleBar`)

- Very thin (~28px) dark bar spanning the full width
- Left side: app icon + Quick Access Toolbar (QAT) icon buttons (Save, New, etc.) — small, no labels
- Centre: window title text (`Winrarhost - Royal TS`)
- Right side: standard Windows min/max/close controls
- **Draggable across the entire bar** (except buttons)
- The tab strip row is SEPARATE from the title bar — they do not merge
- Background: near-black (`#1C1C1C` range), not frosted

### Ribbon Tab Strip (`fluent-ribbon::RibbonTabStrip`)

- Sits below the title bar, full width
- Background: white/light (`#F3F3F3`)
- Tabs are text-only, no icons in the tab label itself
- **Active tab**: bold label text + a solid orange/accent underline at the bottom edge (~2px)
- Inactive tabs: normal weight, no underline
- Tab padding is generous horizontally (~16px each side) but not tall (~28px)
- **Contextual tab** ("Terminal Connection"): the TAB LABEL BACKGROUND is filled with the accent colour (orange), white text — this distinguishes it from normal tabs clearly
- Contextual tab is grouped visually with a thin coloured bar along the TOP of the tab strip above the contextual tabs
- No explicit separator between normal tabs and contextual tabs other than position

### Ribbon Content Row (`fluent-ribbon::RibbonContent`)

- Background: white — visibly lighter than the title bar
- Full-width, fixed height (~80-90px)
- Groups arranged left-to-right in a flex row
- Right edge of each group has a thin vertical separator + a group label centred at the bottom
- Group label text is tiny (caption size, ~11px), muted grey
- A "collapse/expand" toggle (∧) chevron appears at the far right edge of the ribbon content row — collapses the ribbon to just the tab strip

### Ribbon Large Button (`RibbonLargeButton`)

- Height fills the full ribbon content row (~64px content area)
- Layout: icon (32px) centred above, label text below, optionally a dropdown chevron (▾) beneath the label
- Icon is full-colour (not monochrome) in this reference — but FluentGUI should support both
- Width: auto (fits the label)
- Hover: subtle neutral fill

### Ribbon Compact Button (`RibbonButton`)

- Height ~24px, stacks 2–3 per column within a group
- Layout: small icon (16px) + label side by side
- "Close", "Save", "Save All", "Duplicate", "Delete" are compact buttons
- Delete uses a red ✕ icon — coloured icons are supported

### Ribbon Large Button with Dropdown (`RibbonLargeButton` + dropdown)

- "Add" and "New"/"Open" show a chevron beneath the label
- Clicking the chevron opens a flat dropdown list anchored below the button
- Clicking the main body of the button fires the primary action directly
- Dropdown items: icon (16px) + label, flat list, no sub-headers for the short list
- The dropdown is a floating overlay (popover), not inline

### Ribbon Dropdown List (from `Ribbon Dropdown.png`)

- Items: 16px icon + label text, full-width rows, ~28px height each
- Divider between item groups (thin grey line)
- Some items have a right-chevron for cascade (`Command Tasks →`, `More Tasks →`)
- No checkbox / radio states in this reference

### Group Separator (`RibbonSeparator`)

- A thin (~1px) vertical line between groups, full height of the content row
- Colour: light grey (`#E0E0E0` range)
- The group label appears BELOW the separator line centred in the group width

### Navigation Sidebar (`fluent-layout::DockPanel` + tree view)

- Panel header: "Navigation" with a pin (📌) icon and close (✕) icon on the right
- Search box: full-width, placeholder "Enter text to search…", magnifier icon on right
- Tree: indented rows, collapsible with ▶/▼ chevrons, 16px icon + label per row
- Selected item: light accent-tinted row background, bold or normal weight text
- Favourite star badges: inline ★ icon on the right of the row label (orange fill)
- Type icons are coloured (folder = orange, SSH = terminal icon, RDP = screen icon)
- Panel is resizable (drag handle at the right edge)
- Panel can be pinned (auto-hide) or permanently open

### Content Tab Strip (`fluent-layout::TabStrip`)

- Sits BELOW the ribbon, NOT inside the ribbon
- Background same as the sidebar header (~light grey)
- Tabs: icon + label + ✕ close button
- Active tab: white background, contrasts against the grey strip
- Inactive tabs: grey background, lighter text
- Tabs can be scrolled if overflow — there is no explicit overflow button in this reference, just scroll

### Breadcrumb / Path Bar

- Below the content tab strip
- Shows `Winrarhost > SFTP` path for the current session
- Clickable path segments
- This is rdpapp-specific, NOT a framework primitive — lives in rdpapp, not FluentGUI

### Data Entry Dialog (`Generic Data Entry.png`)

- Dialog box (non-fullscreen modal) ~900×700px
- Custom title bar (same dark bar style as main window) — or OS title bar with the app icon
- Left panel (~220px): collapsible section tree
  - Sections: "Remote Desktop", "Common", "Advanced" — with ▼/▶ chevrons
  - Active item has a left-edge accent bar (orange, ~3px)
  - Items have a 16px icon + label
- Right panel (form area):
  - Field label on the LEFT in a fixed column (~120px wide), right-aligned
  - Input field fills the rest of the row
  - **Active/focused input**: bottom-border only in accent colour (orange, ~2px) — NOT a full box border
  - Placeholder text in input: light grey italic
  - Bottom of dialog: creation/modification metadata in muted small text
- No explicit OK/Cancel bar visible in this screenshot — likely at the bottom (outside the crop)

### Right-Click Context Menu (`Right Click Context Menu.png`)

- Flat floating overlay, ~180px wide
- Items: 16px icon + label, ~28px height each
- Separator: thin grey line between item groups
- Cascading submenu: right-chevron on the row, submenu opens to the right
- Selected/hovered item: light blue/neutral fill (full-width row highlight)
- Submenus open on hover, not click, in this reference
- This maps to `fluent-layout::ContextMenu` (Phase 4 addition — not in original plan)

---

## Design Decisions Informed by These Screenshots

### Colour scheme
- These screenshots use an **orange** accent (`#FF8C00` / `#E07400` range) — rdpapp will
  pick its own brand colour. Confirms the token system is correct: accent is a token, not hardcoded.
- Ribbon background is white (`#FFFFFF`), not the neutral grey — `ribbon_bg` token should
  be `surface` in light mode (white), not `neutral`.
- Title bar is very dark, close to `#1C1C1C` in dark mode — matches our dark surface token.

### Ribbon contextual tab treatment
- Contextual tab has a **solid accent-coloured background** on the TAB BUTTON itself (not just
  a stripe above). Update: there is also a thin stripe along the top of the strip above the tab.
- The tab strip row needs to accommodate two visual zones: normal tabs + contextual tab area.
- The contextual area also has a thin accent "header band" above the tab strip — this is a `div`
  with 3–4px height, accent background, spanning only the contextual tab width.

### Ribbon collapse toggle
- A ∧ chevron at the far right of the ribbon collapses/expands the ribbon content row.
- When collapsed: only the tab strip shows. Clicking a tab shows the ribbon temporarily.
- This is a `RibbonBar` state toggle — needs an entity-level `collapsed: bool` field.

### Active input style (dialogs)
- Inputs use a **bottom-border-only** focus indicator (not a full box ring).
- This should be the default `TextInput` focus style — update Phase 2 TextInput.
- The bottom border is accent-coloured; normal (unfocused) state has no bottom border or a
  very subtle grey one.

### Left nav in dialogs
- Active item has a **left edge accent bar** (orange ~3px) flush with the panel edge.
- This is a distinct style from sidebar tree rows — it's more like a settings nav.
- Maps to a `SettingsNav` component (rdpapp-specific, built from fluent-layout primitives).

### Context menu (new Phase 4 addition)
- A `ContextMenu` component was not in the original PLAN.md — it's clearly required.
- Should be added to Phase 4 as `fluent-layout::ContextMenu` alongside the modal stack.
- Needs: anchor position, item list with optional icons, separator, cascade submenu.

### Dual-pane layout (File Explorer)
- Two `Pane`s in a horizontal `PaneGroup`, each with its own toolbar strip.
- Confirms the PaneGroup split model is the right abstraction.
- The column headers (Name, File Type, Modified, Size) with a filter/sort icon are
  rdpapp-specific list views — not framework primitives.

---

## Items Added to Scope

These were observed but were NOT in the original PLAN.md:

| Component | Phase | Notes |
|-----------|-------|-------|
| `RibbonBar` collapse toggle (∧) | Phase 3 | Add `collapsed: bool` state + toggle button |
| Contextual tab accent header band | Phase 3 | Thin stripe above contextual tab region |
| `ContextMenu` (right-click, cascade) | Phase 4 | Add alongside modal stack |
| Bottom-border input focus style | Phase 2 | Refine `TextInput` focus treatment |
| Dialog left-nav with accent bar | Phase 6 | rdpapp-specific `SettingsNav` component |
