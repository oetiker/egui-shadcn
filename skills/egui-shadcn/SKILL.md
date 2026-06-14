---
name: egui-shadcn
description: Use when building or restyling a Rust egui GUI from a shadcn/web design (screenshot, component code, or description). Vendors a tested shadcn-v4 component module into the project and maps the design onto it, layout-first. Backend-agnostic — works with eframe, egui-winit, or any egui integration.
---

# egui-shadcn

Rebuild shadcn-style designs as polished egui GUIs without endless iteration.
The hard part of egui is not styling — it is **layout** (single-pass, no flexbox).
This skill carries a tested component module and a layout-first workflow.

The components are **plain egui widgets**: they only touch `egui::Context`/`Ui`
and depend on nothing beyond `egui` + `egui_extras` (StripBuilder). They drop into
any backend — eframe, `egui-winit` + a custom rasterizer (softbuffer), glow, wgpu.
There is **no eframe or wgpu coupling** in what you ship.

## When to use
A shadcn (or generally web/Tailwind) design needs to become an egui UI: a
screenshot to recreate, shadcn component code to port, or a described screen that
must look professional.

This skill covers two distinct activities — keep them separate:
- **Part A — Using the components** to build a screen (the common case).
- **Part B — Porting a new component** into the library (when a needed widget
  isn't in the registry yet).

---

# Part A — Using the components

1. **Vendor-in the registry.** Copy `registry/src/` and `registry/assets/` from
   this skill into the target project (as a module or sub-crate). Idempotent —
   skip files that already exist and match.
2. **Add the (minimal) deps.** Only two are required, and both are intentionally
   small because they propagate to your whole project:
   - `egui = "0.34"`
   - `egui_extras = { version = "0.34", default-features = false }`
     (StripBuilder/Size only — no image/svg/http loaders)

   **Do not add `eframe` on the registry's account.** Add eframe only if your app
   already uses it as its backend. The components work with whatever egui
   integration you already have (eframe, egui-winit + softbuffer, etc.).
3. **Apply the theme** once per frame at the top of your update/draw:
   `egui_shadcn::Theme::dark().apply(ctx)` (or `light`). Read tokens via
   `Theme::current(ctx)`.
4. **Decompose the design top-down:** app shell → panels
   (`SidePanel`/`TopBottomPanel`/`CentralPanel`); each region's **layout intent**
   (stack? row? grid? space-between?); components + their cva variants; token
   deviations (usually none).
5. **Map** using the reference tables:
   - `references/layout-map.md` — flexbox/Grid intent → egui helper (**read this
     first; layout is where iteration is lost**).
   - `references/component-map.md` — shadcn component/variant → module widget.
   - `references/token-map.md` — shadcn token → egui field.
6. **Build** from the vendored components. Drop to raw egui only when no helper
   fits — and then prefer adding a helper (see Part B) over inlining.

## Verify visually
The web-like feedback loop is: render to a PNG and **look at it** against the
design, then iterate on the numbers. The reference loop is an `egui_kittest`
snapshot test (features `["wgpu","snapshot"]`); generate with
`UPDATE_SNAPSHOTS=1 cargo test`.

This harness is **optional and dev-only** — it is how *this* crate verifies
itself, and the best loop if you can run it. It pulls `egui_kittest` + `wgpu` as
**`[dev-dependencies]`**, which never reach a shipped binary. If your project has
its own render/screenshot path (e.g. a softbuffer rasterizer), use that to capture
the PNG instead — the point is "look at a real render," not this specific harness.

---

# Part B — Porting a new component into the library

When you port a component or layout helper that isn't in the registry yet, do it
to the library's standard so the next project gets it for free.

**A complete contribution adds:**
1. The new file under `src/components/` (or a helper in `src/layout.rs`), matching
   the existing custom-paint pattern in `button.rs`.
2. A `kittest` snapshot test plus its committed PNG.
3. A synced copy under `skills/egui-shadcn/registry/` — the canonical `src/` and
   the registry copy must stay **byte-identical**.
4. A row in `references/component-map.md`.
5. Build + `clippy --all-targets` warning-clean.

Then **offer to open a PR** to `github.com/oetiker/egui-shadcn` (a nudge, not an
automatic pipeline — it needs the user's go-ahead and push/fork access).

## Dependency discipline (important)
Everything in `[dependencies]` propagates to **every consumer and every vendoring
project**. Keep that surface minimal.

- **Default: add no new runtime dependency.** A ported component should be
  expressible with `egui` + `egui_extras` (StripBuilder) + custom painting, like
  every existing one.
- **A new `[dependencies]` entry needs a strong, stated reason** — a capability
  that genuinely cannot be painted or computed in-crate, worth the build cost and
  version-coupling imposed on all consumers. Prefer a tiny focused crate over a
  large one; prefer an *optional, feature-gated* dep over an always-on one. Heavy
  crates (`eframe`, `image`/`resvg` loaders, `wgpu`) must **not** become runtime
  deps — they pull in backends/decoders most consumers don't want.
- **Tooling stays in `[dev-dependencies]`** (`eframe` for the example,
  `egui_kittest` + `wgpu` for snapshots). Those never reach consumers.
- When in doubt, **ask the user before adding a dependency**, and document the
  reason next to it in `Cargo.toml`.

---

## Hard rules (see `references/gotchas.md`)
- egui is single-pass: a widget sets its own size before placement; there is no
  `flex-grow` distribution across siblings. Use `egui_extras::StripBuilder`
  (`Size::relative/exact/remainder`) via the `layout` helpers.
- Don't fight it with `available_width()` arithmetic when a helper exists.
- Hover/active = opacity modulation of the base color, never a new hue.
- Apply the theme every frame; read tokens via `Theme::current(ctx)`.

## Acceptance bar
The reference `egui_shadcn::reference::settings_ui` (a settings screen whose tabs
double as a component gallery) is the quality yardstick: match its restraint —
subtle 1px borders, one radius, muted palette, 14px text, consistent focus rings.
Run `cargo run --example settings` to see it live.
