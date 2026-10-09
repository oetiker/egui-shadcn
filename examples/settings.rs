use egui_shadcn::reference::{settings_ui, SettingsState};
use egui_shadcn::Theme;

struct SettingsApp {
    state: SettingsState,
}

impl eframe::App for SettingsApp {
    // eframe hands us a root &mut Ui; settings_ui fills it via a
    // CentralPanel::show_inside, so no ctx-level panel management is needed.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        settings_ui(ui, &mut self.state);
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "egui-shadcn settings",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            // Once, when the context is created: the theme does not change
            // while the app runs.
            Theme::dark().apply(&cc.egui_ctx);
            Ok(Box::new(SettingsApp { state: SettingsState::default() }))
        }),
    )
}
