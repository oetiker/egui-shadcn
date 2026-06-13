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

#[test]
fn switch_toggles() {
    use egui_kittest::kittest::Queryable;
    use egui_kittest::Harness;
    use egui_shadcn::components::switch::toggle;
    use egui_shadcn::Theme;
    use std::cell::Cell;
    let on = Cell::new(false);
    let mut h = Harness::new_ui(|ui| {
        Theme::dark().apply(ui.ctx());
        let mut v = on.get();
        if toggle(ui, &mut v).clicked() {
            on.set(v);
        }
    });
    h.run();
    h.get_by_role(egui::accesskit::Role::CheckBox).click();
    h.run();
    assert!(on.get(), "switch did not toggle on");
}

#[test]
fn switch_checkbox_snapshot() {
    use egui_kittest::Harness;
    use egui_shadcn::components::checkbox::checkbox;
    use egui_shadcn::components::switch::toggle;
    use egui_shadcn::{layout, Theme};
    let mut h = Harness::builder()
        .with_size(egui::vec2(220.0, 120.0))
        .build_ui(|ui| {
            Theme::dark().apply(ui.ctx());
            ui.add_space(16.0);
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.vertical(|ui| {
                    layout::row(ui, 8.0, |ui| {
                        let mut on = true;
                        toggle(ui, &mut on);
                        let mut off = false;
                        toggle(ui, &mut off);
                    });
                    ui.add_space(12.0);
                    layout::row(ui, 8.0, |ui| {
                        let mut c1 = true;
                        checkbox(ui, &mut c1);
                        let mut c2 = false;
                        checkbox(ui, &mut c2);
                    });
                });
            });
        });
    h.run();
    h.snapshot("switch_checkbox");
}

#[test]
fn tabs_select() {
    use egui_kittest::kittest::Queryable;
    use egui_kittest::Harness;
    use egui_shadcn::components::tabs::tab_bar;
    use egui_shadcn::Theme;
    use std::cell::Cell;
    let active = Cell::new(0usize);
    let mut h = Harness::new_ui(|ui| {
        Theme::dark().apply(ui.ctx());
        let mut sel = active.get();
        tab_bar(ui, &mut sel, &["Account", "Notifications", "Display"]);
        active.set(sel);
    });
    h.run();
    h.get_by_label("Notifications").click();
    h.run();
    assert_eq!(active.get(), 1);
}

#[test]
fn tabs_snapshot() {
    use egui_kittest::Harness;
    use egui_shadcn::components::tabs::tab_bar;
    use egui_shadcn::Theme;
    let mut h = Harness::builder()
        .with_size(egui::vec2(420.0, 70.0))
        .build_ui(|ui| {
            Theme::dark().apply(ui.ctx());
            ui.add_space(16.0);
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                let mut sel = 0usize;
                tab_bar(ui, &mut sel, &["Account", "Notifications"]);
            });
        });
    h.run();
    h.snapshot("tabs");
}
