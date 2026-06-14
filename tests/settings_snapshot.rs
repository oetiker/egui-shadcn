use egui_kittest::Harness;
use egui_shadcn::reference::{settings_ui, SettingsState};
use egui_shadcn::Theme;

/// Render the settings screen with `tab` pre-selected and snapshot it.
fn snapshot_tab(tab: usize, name: &str) {
    let mut state = SettingsState {
        tab,
        name: "Ada Lovelace".into(),
        email: "ada@example.com".into(),
        password: "hunter2".into(),
        marketing_emails: true,
        weekly_digest: true,
        ..Default::default()
    };
    let mut h = Harness::builder()
        .with_size(egui::vec2(640.0, 720.0))
        .build_ui(move |ui| {
            Theme::dark().apply(ui.ctx());
            settings_ui(ui, &mut state);
        });
    h.run();
    h.snapshot(name);
}

#[test]
fn settings_account_tab() {
    snapshot_tab(0, "settings_account");
}

#[test]
fn settings_notifications_tab() {
    snapshot_tab(1, "settings_notifications");
}

#[test]
fn settings_components_tab() {
    snapshot_tab(2, "settings_components");
}
