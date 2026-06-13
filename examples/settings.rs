use egui_shadcn::reference::{settings_ui, SettingsState};
use egui_shadcn::Theme;

fn main() -> eframe::Result<()> {
    let mut state = SettingsState::default();
    // run_ui_native provides &mut Ui instead of &Context; since settings_ui
    // owns its CentralPanel it needs ctx access — use run_simple_native.
    #[allow(deprecated)]
    eframe::run_simple_native(
        "egui-shadcn settings",
        eframe::NativeOptions::default(),
        move |ctx, _frame| {
            Theme::dark().apply(ctx);
            settings_ui(ctx, &mut state);
        },
    )
}
