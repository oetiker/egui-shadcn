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
| shadow-sm | `Frame::shadow` (offset [0,1], blur 3) |

Tokens are OKLCH literals converted by `color::oklch_to_srgb` into the `Palette`.
Customize by editing `Palette::light()/dark()` in the vendored `theme.rs`.
