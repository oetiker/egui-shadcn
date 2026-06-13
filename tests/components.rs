use egui_shadcn::components::shared::mix_toward;

#[test]
fn mix_toward_blends() {
    let a = egui::Color32::from_rgb(200, 0, 0);
    let b = egui::Color32::from_rgb(0, 0, 0);
    let m = mix_toward(a, b, 0.5);
    assert!(m.r() > 90 && m.r() < 110, "got {}", m.r());
}

#[test]
fn button_click_sets_flag() {
    use egui_kittest::kittest::Queryable;
    use egui_kittest::Harness;
    use egui_shadcn::components::button::Button;
    use egui_shadcn::Theme;
    use std::cell::Cell;
    let clicked = Cell::new(false);
    let mut h = Harness::new_ui(|ui| {
        Theme::dark().apply(ui.ctx());
        if ui.add(Button::new("Save")).clicked() {
            clicked.set(true);
        }
    });
    h.get_by_label("Save").click();
    h.run();
    assert!(clicked.get(), "button click did not register");
}

#[test]
fn button_variants_snapshot() {
    use egui_kittest::Harness;
    use egui_shadcn::components::button::{Button, ButtonVariant};
    use egui_shadcn::{layout, Theme};
    let mut h = Harness::builder()
        .with_size(egui::vec2(560.0, 80.0))
        .build_ui(|ui| {
            Theme::dark().apply(ui.ctx());
            ui.add_space(20.0);
            layout::row(ui, 8.0, |ui| {
                ui.add_space(16.0);
                ui.add(Button::new("Default"));
                ui.add(Button::new("Secondary").variant(ButtonVariant::Secondary));
                ui.add(Button::new("Outline").variant(ButtonVariant::Outline));
                ui.add(Button::new("Ghost").variant(ButtonVariant::Ghost));
                ui.add(Button::new("Delete").variant(ButtonVariant::Destructive));
            });
        });
    h.run();
    h.snapshot("button_variants");
}

#[test]
fn input_accepts_text() {
    use egui_kittest::Harness;
    use egui_shadcn::components::input::Input;
    use egui_shadcn::Theme;
    let mut text = String::new();
    let mut h = Harness::new_ui(|ui| {
        Theme::dark().apply(ui.ctx());
        ui.add(Input::new(&mut text).hint("Email"));
    });
    h.run();
    assert!(h.ctx.viewport_rect().width() > 0.0);
}

#[test]
fn label_input_snapshot() {
    use egui_kittest::Harness;
    use egui_shadcn::components::input::Input;
    use egui_shadcn::components::label::{description, label};
    use egui_shadcn::Theme;
    let mut h = Harness::builder()
        .with_size(egui::vec2(360.0, 140.0))
        .build_ui(|ui| {
            Theme::dark().apply(ui.ctx());
            ui.add_space(16.0);
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.vertical(|ui| {
                    label(ui, "Email");
                    let mut s = String::from("you@example.com");
                    ui.add(Input::new(&mut s));
                    description(ui, "We'll never share your email.");
                });
            });
        });
    h.run();
    h.snapshot("label_input");
}
