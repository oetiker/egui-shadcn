//! shadcn Tooltip: a small inverted label (primary fill, primary-foreground
//! text, `text-xs`, `px-3 py-1.5`) shown while hovering a widget.

use crate::components::shared::corner;
use crate::Theme;
use egui::{Frame, Margin, Response, RichText};

/// Attach a shadcn tooltip to `resp`: `tooltip(ui.add(..), "Mute")`.
pub fn tooltip(resp: Response, text: &str) -> Response {
    let t = Theme::current(&resp.ctx);
    let (p, m) = (&t.palette, &t.metrics);
    let mut tip = egui::Tooltip::for_enabled(&resp);
    tip.popup = tip.popup.frame(
        Frame::new()
            .fill(p.primary)
            .corner_radius(corner(t.radius_md()))
            .inner_margin(Margin::symmetric(m.pad_sm as i8, (m.gap * 0.75) as i8)),
    );
    tip.show(|ui| ui.label(RichText::new(text).size(m.text_xs).color(p.primary_foreground)));
    resp
}
