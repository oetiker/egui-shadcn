use egui_kittest::Harness;
use egui_shadcn::reference::{settings_ui, SettingsState};
use egui_shadcn::Theme;

#[test]
fn settings_account_tab() {
    let mut state = SettingsState::default();
    // builder().build() is deprecated in egui 0.34 but is the only way to get
    // a ctx-level closure (needed for CentralPanel) with a custom size.
    #[allow(deprecated)]
    let mut h = Harness::builder()
        .with_size(egui::vec2(640.0, 600.0))
        .build(move |ctx| {
            Theme::dark().apply(ctx);
            settings_ui(ctx, &mut state);
        });
    h.run();
    h.snapshot("settings_account");
}
