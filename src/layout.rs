//! Flexbox substitutes for egui's single-pass layout.
//! `gap` maps to item_spacing; grow/space-between use egui_extras::StripBuilder.

use egui::Ui;
use egui_extras::{Size, StripBuilder};

/// Horizontal stack with an explicit gap (flex-direction: row; gap: N).
pub fn row<R>(ui: &mut Ui, gap: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = gap;
        add(ui)
    })
    .inner
}

/// Vertical stack with an explicit gap (flex-direction: column; gap: N).
pub fn vstack<R>(ui: &mut Ui, gap: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = gap;
        add(ui)
    })
    .inner
}

/// A row whose `left` content hugs the start and `right` content hugs the end
/// (justify-content: space-between). Implemented with two remainder cells.
pub fn space_between(
    ui: &mut Ui,
    left: impl FnOnce(&mut Ui),
    right: impl FnOnce(&mut Ui),
) {
    StripBuilder::new(ui)
        .size(Size::remainder())
        .size(Size::remainder())
        .horizontal(|mut strip| {
            strip.cell(|ui| {
                ui.horizontal(|ui| left(ui));
            });
            strip.cell(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    right(ui)
                });
            });
        });
}
