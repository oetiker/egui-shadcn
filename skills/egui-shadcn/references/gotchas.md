# egui gotchas (why web habits fail)

- **Single pass, no negotiation.** A widget commits its size before the parent's
  size is known. No `flex-grow` across siblings, no intrinsic min/max-content.
  Use `StripBuilder` for distribution.
- **`gap` is `item_spacing`.** Set it inside the stack closure; it doesn't cascade
  like CSS.
- **`StripBuilder.horizontal` eats all vertical space.** Wrap in `allocate_ui` with
  a fixed row height.
- **First-frame jitter & deferred fonts.** `ctx.set_fonts` takes effect next frame,
  so a named font family may be missing on frame 1 — use `theme::family(ctx, name)`
  which falls back to Proportional until the atlas is ready. For snapshot tests,
  `harness.run()` steps frames before snapshotting.
- **`override_text_color` is baked into galleys.** `WidgetText::into_galley` bakes
  the global text color, so `painter().galley(.., color)` can't override it. To
  force a per-widget color (e.g. dark text on a light button), build the galley
  with `painter().layout_no_wrap(text, font_id, explicit_color)` (see `button.rs`).
- **`strong()` is not a bold font.** egui's `strong` brightens color. For weight use the
  named families via `theme::family(ctx, FAMILY_MEDIUM|FAMILY_SEMIBOLD)`.
- **No CSS transitions.** Animate manually with `ctx.animate_bool(id, on)` and lerp
  (see `switch.rs`).
- **No multi-stop/radial gradients, no per-widget box-shadow stacks.** One `Shadow`
  per `Frame`; gradients need a hand-built `Mesh`.
- **Apply theme every frame.** `Theme::apply` installs fonts once (guarded) and sets
  style; read tokens via `Theme::current(ctx)` inside widgets.
