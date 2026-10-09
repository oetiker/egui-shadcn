# shadcn token → egui mapping

| shadcn token | egui target |
|---|---|
| `--background` | `Visuals.panel_fill`; `Palette.background` |
| `--foreground` | `Visuals.override_text_color`; `Palette.foreground` |
| `--card` / `--card-foreground` | `layout::card` fill / text |
| `--popover` | ComboBox/menu surfaces (`window_fill`) |
| `--primary` / `-foreground` | Button Default fill / text |
| `--secondary` / `-foreground` | Button Secondary; `widgets.*.bg_fill` |
| `--muted` / `-foreground` | Tabs track; description text |
| `--accent` | Ghost hover fill; native widget hover |
| `--destructive` | Button Destructive fill |
| `--border` | 1px strokes everywhere; `widgets.*.bg_stroke` |
| `--input` | TextEdit bg; switch off-track |
| `--ring` | focus ring (3px @ 50%) |
| `--radius` (10px) | `Theme.radius`; `radius_sm/md/lg/xl` |
| 4px spacing grid | `Spacing.item_spacing`, margins |
| 14px / medium | `text_styles[Body]` + `theme::family(ctx, FAMILY_MEDIUM)` |
| shadow-sm | `Frame::shadow` (offset [0,1], blur 3), color `Palette.shadow` |
| `text-xs` / `text-sm` / `text-base` | `Metrics.text_xs` / `text_sm` / `text_base` |
| `h-8` / `h-9` / `h-10` | `Metrics.control_sm` / `control_md` / `control_lg` |
| `px-3` / `px-4` / `px-6` | `Metrics.pad_sm` / `pad_md` / `pad_lg` |
| `gap-2` | `Metrics.gap` (also `Spacing.item_spacing`) |
| `p-6` (card) | `Metrics.card_padding` |
| `size-4` (icons, checkbox) | `Metrics.icon` |
| `border` / `ring-[3px]` widths | `Metrics.border` / `Metrics.ring` |

Tokens are OKLCH literals converted by `color::oklch_to_srgb` into the `Palette`.
Customize by editing `Palette::light()/dark()` in the vendored `theme.rs`, or
add your own preset there (a brand palette is one more `Palette` constructor).
Sizes: `Theme { metrics: Metrics { control_md: 32.0, ..Metrics::default() }, ..Theme::dark() }`.

**Colors and text sizes live in `theme.rs` and nowhere else.** A widget that
writes `Color32::from_rgb(..)` or `FontId::proportional(14.0)` drifts from the
theme the first time the palette changes; that is how hand-built screens end up
with a navy panel inside a green app. `tests/tokens.rs` enforces this for the
registry; vendoring projects should run the same scan over their own UI files
(see SKILL.md, Acceptance bar).
