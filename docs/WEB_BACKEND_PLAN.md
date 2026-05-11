# FluentGUI Web Backend — High-Level Plan

**Objective**: Run FluentGUI applications unmodified in a browser via a WebAssembly/WebGPU GPUI backend.

**Key insight**: FluentGUI itself requires zero changes. All work lives in GPUI's platform and renderer layers, which are fully Apache-2.0 licensed and already partially implemented in the Zed codebase.

---

## Licensing Baseline

Everything needed is Apache-2.0 in the Zed repo:

| Crate | What it is |
|-------|-----------|
| `gpui` | Core framework — traits, scene, layout, element system |
| `gpui_web` | Existing web platform implementation (2,552 LOC) |
| `gpui_wgpu` | wgpu-based renderer used by Linux, Windows, and web (3,486 LOC) |
| `gpui_macos` | Metal renderer (macOS) |
| `gpui_linux` | Wayland/X11 platform |
| `scheduler` | Task scheduling |

GPL-licensed Zed crates (`ui`, `workspace`, `pane`, `dock`, `editor`, etc.) are never touched.

---

## What Already Exists

`gpui_web` + `gpui_wgpu` together implement:

- Full `Platform` + `PlatformWindow` trait implementations targeting `wasm32-unknown-unknown`
- wgpu renderer covering all 8 scene primitive types (Shadow, Quad, Path, Underline, MonochromeSprite, SubpixelSprite, PolychromeSprite, Surface)
- Sprite atlas backed by wgpu textures
- Browser event translation (mouse, keyboard, scroll, resize) → GPUI `PlatformInput`
- `requestAnimationFrame`-based foreground executor
- Clipboard via the browser Clipboard API
- `cosmic-text` based text shaping (wasm-compatible)

These crates are the foundation. The work below is gap-filling, integration, and publishing — not building from scratch.

---

## Work Breakdown (High Level)

### Phase 1 — Audit & Gap Analysis
*For detailed agent investigation*

1. **Build audit**: Can `gpui_web` + `gpui_wgpu` compile to `wasm32-unknown-unknown` today, as-is from the Zed repo? What errors appear? What dependencies block it?
2. **Feature completeness**: Which `Platform` / `PlatformWindow` / `PlatformTextSystem` trait methods are stubbed vs. fully implemented in `gpui_web`? List every `todo!()`, `unimplemented!()`, or empty body.
3. **Background executor**: `gpui`'s `BackgroundExecutor` assumes OS threads. On WASM there are no threads without web workers. Determine the exact call sites and whether stubbing to synchronous execution breaks anything FluentGUI uses.
4. **`resvg` system-fonts feature**: This pulls in platform font APIs. Determine if it is reachable from the WASM compile path and what needs feature-gating.
5. **Subpixel rendering**: `SubpixelSprite` primitives use RGB subpixel rendering. Determine if WebGPU shader precision supports this or if it must be disabled/degraded on web.
6. **Bundle size baseline**: Compile a minimal `gpui_web` app to WASM and measure the `.wasm` size before and after `wasm-opt`. Determine if it is acceptable for a web app.

---

### Phase 2 — Standalone Crate Publishing
*Prerequisite for FluentGUI to depend on the web backend*

The `gpui_web` and `gpui_wgpu` crates currently live inside the Zed monorepo and are not published to crates.io. To use them from FluentGUI:

**Option A**: Fork and publish them as `gpui-web` and `gpui-wgpu` on crates.io under the Apache-2.0 license, matching the GPUI 0.2.2 version pin that FluentGUI already uses.

**Option B**: Contribute the crates upstream to the `gpui` crates.io release so they become first-party published crates.

**Option C**: Use a git dependency pointing at the Zed repo at a pinned commit (acceptable for development, not ideal for published crates).

Decision point: Option A is fastest. Option B is cleanest long-term. Investigate whether the Zed team publishes these or intends to.

---

### Phase 3 — Integration & Missing Pieces

Based on the gap analysis from Phase 1, implement whatever is missing. Expected items (to be confirmed by agent investigation):

1. **Font loading for web**
   - fontconfig and Core Text are not available in WASM
   - `cosmic-text` handles shaping; the gap is font discovery and loading
   - Approach: bundle fonts as `include_bytes!` in the application binary, or load via `fetch()` at startup with a loading screen
   - `PlatformTextSystem::add_fonts()` already accepts raw font bytes — the wiring exists

2. **Background executor**
   - If FluentGUI apps use `cx.background_executor()` for async work (file I/O, network), this needs a real implementation
   - Approach A: Web Workers (correct, complex — requires `wasm-bindgen-rayon` or manual worker pool)
   - Approach B: Stub to `wasm-bindgen-futures` single-threaded execution (sufficient if apps only do UI-driven async)
   - Determine which FluentGUI uses

3. **Platform no-ops for web**
   - `prompt_for_paths()` → `<input type="file">` via web-sys
   - `write_credentials()` / `read_credentials()` → `localStorage` or stub
   - `open_url()` → `window.open()`
   - `reveal_path()`, `open_with_system()` → no-op
   - Dock menu, app menu, jump list → no-op

4. **Frame timing**
   - wgpu's web surface requires the `SurfaceTexture` to be presented within the same RAF callback that acquired it
   - Audit `WgpuRenderer::draw()` call site in `gpui_web` to confirm this is handled correctly

---

### Phase 4 — FluentGUI Wiring

Once the GPUI web backend builds and runs:

1. Add `wasm32-unknown-unknown` as a supported target in FluentGUI's CI
2. Add a minimal web example (e.g., `examples/hello_world` compiled to WASM, served via `trunk` or `wasm-pack`)
3. Confirm all 5 published FluentGUI crates compile to WASM without modification
4. Confirm the `connect_demo` example runs in a browser (golden path test)
5. Document the web build process in README

---

### Phase 5 — Polish & Distribution

1. **Bundle size optimization**: `wasm-opt` passes, feature-flag heavy dependencies, lazy font loading
2. **Subpixel rendering fallback**: If `SubpixelSprite` doesn't work on WebGPU, render glyphs as monochrome and confirm text quality is acceptable
3. **CI integration**: Add a WASM build step to `.woodpecker.yml` (compile-check only, no headless browser test needed initially)
4. **GitHub Pages demo**: Deploy the `connect_demo` example as a live web demo from the GitHub mirror

---

## Key Open Questions (for agent investigation)

These are unknowns that block accurate effort estimation. Each should be a discrete agent task:

| # | Question | Why it matters |
|---|----------|----------------|
| 1 | Does `gpui_web` compile to `wasm32-unknown-unknown` today without modifications? | Determines if Phase 1 is a few fixes or a major undertaking |
| 2 | How many `Platform` / `PlatformWindow` methods are unimplemented/stubbed in `gpui_web`? | Determines Phase 3 scope |
| 3 | Does FluentGUI's application code use `cx.background_executor()` for anything? | Determines if web worker complexity is required |
| 4 | What is the WASM binary size of a minimal `gpui_web` app? | Determines viability for web deployment |
| 5 | Does the Zed team publish `gpui_web` / `gpui_wgpu` to crates.io, or plan to? | Determines Phase 2 approach |
| 6 | Is `SubpixelSprite` rendering functional under wgpu's WebGPU backend? | Determines if text quality degrades on web |

---

## Risk Register

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|-----------|
| `gpui_web` has significant unimplemented sections | Medium | High | Phase 1 audit before committing to Phase 3 |
| Background executor requires web workers | Medium | Medium | Audit FluentGUI's actual background executor usage first |
| WASM bundle too large for practical web use | Low | Medium | `wasm-opt` + lazy loading covers most cases |
| Subpixel rendering broken on WebGPU | Medium | Low | Monochrome fallback is acceptable for most text |
| Zed changes `gpui_web` incompatibly with 0.2.2 | Low | Medium | Pin to a specific Zed commit when forking |
| `resvg` system-fonts blocks WASM compile | Medium | Medium | Feature-gate; use `usvg` directly on web |

---

## Sequencing

```
Phase 1 (Audit)
    ├── Q1: Does gpui_web compile to wasm today?
    ├── Q2: What is stubbed/unimplemented?
    ├── Q3: Background executor usage in FluentGUI?
    ├── Q4: WASM bundle size?
    └── Q5: Crates.io publishing status?
          │
          ▼
Phase 2 (Publishing)        Phase 3 (Gap filling — parallel once Phase 1 done)
    └── Fork + publish           ├── Font loading
         gpui-web, gpui-wgpu     ├── Background executor
                                 ├── Platform no-ops
                                 └── Frame timing audit
                │
                ▼
         Phase 4 (FluentGUI wiring)
                │
                ▼
         Phase 5 (Polish + demo)
```

---

## Success Criteria

- `cargo build --target wasm32-unknown-unknown` passes for all FluentGUI crates
- `connect_demo` example runs in a modern browser (Chrome/Firefox/Safari with WebGPU)
- Text renders correctly (monochrome acceptable, subpixel preferred)
- Dark/light theme switching works
- No changes required to FluentGUI application code
