//! shadcn Dialog: a modal card over a dimmed backdrop, with title,
//! description, a close cross, and a right-aligned footer.
//!
//! Built on `egui::Modal`, which keeps backdrop and card in one area (so their
//! z-order cannot flip) and blocks input to everything beneath. Closes on Esc,
//! a backdrop click, or the cross.

use crate::components::icon::{paint_icon, CLOSE};
use crate::components::shared::corner;
use crate::theme::{family, FAMILY_SEMIBOLD};
use crate::{layout, Theme};
use egui::{Color32, Frame, Id, Margin, RichText, Sense, Stroke, Ui, Vec2};

pub struct Dialog<'a> {
    id: Id,
    title: &'a str,
    description: Option<&'a str>,
    width: Option<f32>,
    backdrop: Option<Color32>,
}

impl<'a> Dialog<'a> {
    pub fn new(id_salt: impl std::hash::Hash, title: &'a str) -> Self {
        Self { id: Id::new(id_salt), title, description: None, width: None, backdrop: None }
    }
    pub fn description(mut self, text: &'a str) -> Self {
        self.description = Some(text);
        self
    }
    /// Card width; default `Metrics::dialog_width`, capped to the window.
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
    /// Backdrop color; default `Palette::overlay`. Under a CPU rasterizer a
    /// translucent full-window backdrop is re-blended every frame the dialog
    /// is open; pass `Color32::TRANSPARENT` and dim the background once in the
    /// compositor instead (see `cpu-rendering.md`). It still blocks clicks.
    pub fn backdrop(mut self, color: Color32) -> Self {
        self.backdrop = Some(color);
        self
    }

    /// Show the dialog while `*open`; clears `open` when the user dismisses it.
    /// Returns the body's value while open.
    pub fn show<R>(self, ctx: &egui::Context, open: &mut bool, body: impl FnOnce(&mut Ui) -> R) -> Option<R> {
        if !*open {
            return None;
        }
        let t = Theme::current(ctx);
        let (p, m) = (&t.palette, &t.metrics);
        let screen = ctx.content_rect();
        let width = self.width.unwrap_or(m.dialog_width).min(screen.width() - 2.0 * m.pad_md);
        let frame = Frame::new()
            .fill(p.background)
            .stroke(Stroke::new(m.border, p.border))
            .corner_radius(corner(t.radius_lg()))
            .inner_margin(Margin::same(m.card_padding as i8))
            .shadow(t.shadow_lg());

        let mut close = false;
        let resp = egui::Modal::new(self.id)
            .backdrop_color(self.backdrop.unwrap_or(p.overlay))
            .frame(frame)
            .show(ctx, |ui| {
                ui.set_width(width);
                ui.spacing_mut().item_spacing.y = m.gap;
                layout::space_between(
                    ui,
                    |ui| {
                        let fam = family(ui.ctx(), FAMILY_SEMIBOLD);
                        ui.label(RichText::new(self.title).family(fam).size(m.text_lg).color(p.foreground));
                    },
                    |ui| close = close_cross(ui).clicked(),
                );
                if let Some(d) = self.description {
                    ui.label(RichText::new(d).size(m.text_sm).color(p.muted_foreground));
                }
                ui.add_space(m.gap);
                body(ui)
            });
        if close || resp.should_close() {
            *open = false;
        }
        Some(resp.inner)
    }
}

/// The dialog's close cross: `size-4`, muted until hovered.
fn close_cross(ui: &mut Ui) -> egui::Response {
    let t = Theme::current(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(t.metrics.icon), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), "Close"));
    let color = if resp.hovered() { t.palette.foreground } else { t.palette.muted_foreground };
    paint_icon(ui.painter(), rect, &CLOSE, color, t.metrics.icon_stroke);
    crate::components::shared::focus_ring(ui, &resp, &t, t.radius_sm());
    resp
}

/// Dialog footer: actions aligned right with `gap-2`, primary action last
/// (add it first: the row runs right to left).
pub fn footer(ui: &mut Ui, add: impl FnOnce(&mut Ui)) {
    let gap = Theme::current(ui.ctx()).metrics.gap;
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.spacing_mut().item_spacing.x = gap;
        add(ui);
    });
}
