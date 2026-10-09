//! shadcn Badge: small pill, text-xs (12px), rounded-full.

use crate::Theme;
use egui::{Color32, CornerRadius, FontId, Sense, StrokeKind, Ui, Vec2};

#[derive(Clone, Copy)]
pub enum BadgeVariant {
    Default,
    Secondary,
    Destructive,
    Outline,
}

pub fn badge(ui: &mut Ui, text: &str, variant: BadgeVariant) -> egui::Response {
    let t = Theme::current(ui.ctx());
    let p = &t.palette;
    let (fill, fg, border) = match variant {
        BadgeVariant::Default => (p.primary, p.primary_foreground, None),
        BadgeVariant::Secondary => (p.secondary, p.secondary_foreground, None),
        BadgeVariant::Destructive => (p.destructive, p.destructive_foreground, None),
        BadgeVariant::Outline => (Color32::TRANSPARENT, p.foreground, Some(p.border)),
    };
    let font = FontId::proportional(t.metrics.text_xs);
    let galley = ui.painter().layout_no_wrap(text.to_owned(), font.clone(), fg);
    // shadcn badge: `px-2 py-0.5`, a pill 20px tall.
    let size = Vec2::new(galley.size().x + 16.0, 20.0);
    let (rect, resp) = ui.allocate_exact_size(size, Sense::hover());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text));
    let radius = CornerRadius::same((rect.height() / 2.0) as u8);
    if fill != Color32::TRANSPARENT {
        ui.painter().rect_filled(rect, radius, fill);
    }
    if let Some(b) = border {
        ui.painter().rect_stroke(rect, radius, egui::Stroke::new(t.metrics.border, b), StrokeKind::Inside);
    }
    ui.painter().galley(rect.center() - galley.size() / 2.0, galley, fg);
    resp
}
