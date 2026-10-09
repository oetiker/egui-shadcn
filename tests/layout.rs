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

#[test]
fn wrap_row_wraps_when_out_of_width() {
    use std::cell::RefCell;
    let rects = RefCell::new(Vec::new());
    let mut h = Harness::builder()
        .with_size(egui::vec2(200.0, 200.0))
        .build_ui(|ui| {
            Theme::dark().apply(ui.ctx());
            rects.borrow_mut().clear();
            layout::wrap_row(ui, 8.0, |ui| {
                for _ in 0..4 {
                    let (r, _) = ui.allocate_exact_size(egui::vec2(80.0, 20.0), egui::Sense::hover());
                    rects.borrow_mut().push(r);
                }
            });
        });
    h.run();
    let r = rects.borrow();
    assert_eq!(r[0].top(), r[1].top(), "first two items share a line");
    assert!(r[2].top() >= r[0].bottom() + 8.0, "third item wraps with the vertical gap: {:?}", *r);
    assert_eq!(r[1].left() - r[0].right(), 8.0, "horizontal gap");
}
