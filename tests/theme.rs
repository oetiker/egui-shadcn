use egui_shadcn::theme::Theme;

#[test]
fn apply_sets_panel_fill_and_stores_theme() {
    use egui_shadcn::theme::{theme, Theme};
    let ctx = egui::Context::default();
    Theme::dark().apply(&ctx);
    // run one frame so style takes effect
    let _ = ctx.run_ui(Default::default(), |_ui| {});
    let got = theme(&ctx);
    assert!(got.dark);
    assert_eq!(ctx.global_style().visuals.panel_fill, Theme::dark().palette.background);
}

#[test]
fn radius_scale_is_derived() {
    let t = Theme::dark();
    assert_eq!(t.radius, 10.0);
    assert_eq!(t.radius_sm(), 6.0);
    assert_eq!(t.radius_md(), 8.0);
    assert_eq!(t.radius_lg(), 10.0);
    assert_eq!(t.radius_xl(), 14.0);
}

#[test]
fn light_and_dark_differ() {
    let l = Theme::light();
    let d = Theme::dark();
    assert!(!l.dark && d.dark);
    assert_ne!(l.palette.background, d.palette.background);
}

#[test]
fn dark_border_is_low_alpha_white() {
    let d = Theme::dark();
    // dark --border is oklch(1 0 0 / 10%) -> white at ~26 alpha
    assert_eq!(d.palette.border.a(), 26);
}
