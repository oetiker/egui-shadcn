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
