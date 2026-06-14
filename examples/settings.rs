use egui_shadcn::reference::{settings_ui, SettingsState};
use egui_shadcn::Theme;

fn main() -> eframe::Result<()> {
    let mut state = SettingsState::default();
    // run_ui_native hands us a root &mut Ui; settings_ui fills it via a
    // CentralPanel::show_inside, so no ctx-level panel management is needed.
    eframe::run_ui_native(
        "egui-shadcn settings",
        eframe::NativeOptions::default(),
        move |ui, _frame| {
            Theme::dark().apply(ui.ctx());
            settings_ui(ui, &mut state);
        },
    )
}
