use egui_kittest::Harness;
use egui_shadcn::{layout, Theme};

#[test]
fn row_and_vstack_compose() {
    let mut h = Harness::new_ui(|ui| {
        Theme::dark().apply(ui.ctx());
        layout::vstack(ui, 12.0, |ui| {
            ui.label("top");
            layout::row(ui, 8.0, |ui| {
                ui.label("a");
                ui.label("b");
            });
        });
    });
    h.run();
    assert!(h.ctx.viewport_rect().width() > 0.0);
}

#[test]
fn space_between_composes() {
    let mut h = Harness::new_ui(|ui| {
        Theme::dark().apply(ui.ctx());
        layout::space_between(
            ui,
            |ui| {
                ui.label("left");
            },
            |ui| {
                ui.label("right");
            },
        );
    });
    h.run();
    assert!(h.ctx.viewport_rect().width() > 0.0);
}

#[test]
fn card_and_form_row_render() {
    let mut h = Harness::builder()
        .with_size(egui::vec2(440.0, 180.0))
        .build_ui(|ui| {
            Theme::dark().apply(ui.ctx());
            ui.add_space(16.0);
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.allocate_ui(egui::vec2(380.0, 120.0), |ui| {
                    layout::card(ui, |ui| {
                        layout::form_row(ui, 100.0, "Name", |ui| {
                            let mut s = String::from("hi");
                            ui.text_edit_singleline(&mut s);
                        });
                    });
                });
            });
        });
    h.run();
    h.snapshot("layout_card");
}
