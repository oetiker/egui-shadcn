---
name: egui-shadcn
description: Use when building or restyling a Rust egui/eframe GUI from a shadcn/web design (screenshot, component code, or description). Vendors a tested shadcn-v4 component module into the project and maps the design onto it, layout-first.
---

# egui-shadcn

Rebuild shadcn-style designs as polished egui GUIs without endless iteration.
The hard part of egui is not styling — it is **layout** (single-pass, no flexbox).
This skill carries a tested component module and a layout-first workflow.

## When to use
A shadcn (or generally web/Tailwind) design needs to become an egui/eframe UI:
a screenshot to recreate, shadcn component code to port, or a described screen
that must look professional.

## Workflow

1. **Vendor-in the registry.** Copy `registry/src/` and `registry/assets/` from this
   skill into the target project (as a module or sub-crate), and add deps:
   `egui = "0.34"`, `eframe = "0.34"`, `egui_extras = { version = "0.34",
   features = ["all_loaders"] }`. Call `egui_shadcn::Theme::dark().apply(ctx)`
   (or `light`) once per frame at the top of your update. Idempotent — skip files
   that already exist and match.
2. **Decompose the design top-down:**
   - App shell → egui panels (`SidePanel`/`TopBottomPanel`/`CentralPanel`).
   - Each region's **layout intent** (stack? row? grid? space-between?).
   - Components + their cva variants.
   - Token deviations from the defaults (usually none).
3. **Map** using the reference tables:
   - `references/layout-map.md` — flexbox/Grid intent → egui helper (**read this
     first; layout is where iteration is lost**).
   - `references/component-map.md` — shadcn component/variant → module widget.
   - `references/token-map.md` — shadcn token → egui field.
4. **Build** from the vendored components. Only drop to raw egui when no helper
   fits — and when you do, add a new helper to `layout.rs`/`components/` rather
   than inlining, so the registry keeps growing.
5. **Verify visually.** Add an `egui_kittest` snapshot test
   (features `["wgpu","snapshot"]`), render to PNG, and **look at the PNG**
   against the design. Iterate on the numbers. This is the web-like feedback loop —
   use it instead of guessing. (Generate with `UPDATE_SNAPSHOTS=1 cargo test`.)

## Hard rules (see `references/gotchas.md`)
- egui is single-pass: a widget sets its own size before placement; there is no
  `flex-grow` distribution across siblings. Use `egui_extras::StripBuilder`
  (`Size::relative/exact/remainder`) via the `layout` helpers.
- Don't fight it with `available_width()` arithmetic when a helper exists.
- Hover/active = opacity modulation of the base color, never a new hue.
- Apply the theme every frame; read tokens via `Theme::current(ctx)`.

## Acceptance bar
The reference `egui_shadcn::reference::settings_ui` (settings form with tabs) is
the quality yardstick: match its restraint — subtle 1px borders, one radius,
muted palette, 14px text, consistent focus rings. Run `cargo run --example
settings` to see it live.
