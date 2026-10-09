//! shadcn Card section helpers: title + description headers (use inside layout::card).

use crate::components::label::description;
use crate::theme::{family, FAMILY_SEMIBOLD};
use egui::{RichText, Ui};

pub fn card_title(ui: &mut Ui, text: &str) {
    let fam = family(ui.ctx(), FAMILY_SEMIBOLD);
    let size = crate::Theme::current(ui.ctx()).metrics.text_base;
    ui.label(RichText::new(text).family(fam).size(size));
}

pub fn card_description(ui: &mut Ui, text: &str) {
    description(ui, text);
}
