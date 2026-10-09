//! shadcn Popover: a floating card anchored to a trigger, toggled by clicking
//! it, closed by clicking outside or Esc.
//!
//! egui remembers one open popup at a time, so opening a popover closes any
//! other popup (a menu, a combo box). Nest them only through `open_bool`
//! state of your own.

use crate::components::shared::corner;
use crate::Theme;
use egui::{Frame, Margin, PopupCloseBehavior, Response, Stroke, Ui};

pub struct Popover<'a> {
    trigger: &'a Response,
    width: Option<f32>,
}

impl<'a> Popover<'a> {
    /// A popover toggled by clicks on `trigger`.
    pub fn new(trigger: &'a Response) -> Self {
        Self { trigger, width: None }
    }
    /// Content width; default `Metrics::popover_width`.
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
    /// Show the content while open. Returns its value while open.
    pub fn show<R>(self, content: impl FnOnce(&mut Ui) -> R) -> Option<R> {
        let t = Theme::current(&self.trigger.ctx);
        let (p, m) = (&t.palette, &t.metrics);
        let frame = Frame::new()
            .fill(p.popover)
            .stroke(Stroke::new(m.border, p.border))
            .corner_radius(corner(t.radius_md()))
            .inner_margin(Margin::same(m.pad_md as i8))
            .shadow(t.shadow_md());
        let width = self.width.unwrap_or(m.popover_width);
        egui::Popup::from_toggle_button_response(self.trigger)
            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
            .gap(m.gap / 2.0)
            .frame(frame)
            .width(width)
            .show(|ui| {
                ui.set_width(width);
                ui.visuals_mut().override_text_color = Some(p.popover_foreground);
                content(ui)
            })
            .map(|r| r.inner)
    }
}
