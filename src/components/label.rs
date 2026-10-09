//! shadcn Label: small medium-weight text; plus a muted description helper.

use crate::theme::FAMILY_MEDIUM;
use egui::{RichText, Ui};

pub fn label(ui: &mut Ui, text: &str) -> egui::Response {
    let family = crate::theme::family(ui.ctx(), FAMILY_MEDIUM);
    let size = crate::Theme::current(ui.ctx()).metrics.text_sm;
    ui.label(RichText::new(text).family(family).size(size))
}

pub fn description(ui: &mut Ui, text: &str) -> egui::Response {
    let t = crate::Theme::current(ui.ctx());
    ui.label(RichText::new(text).size(t.metrics.text_sm).color(t.palette.muted_foreground))
}
