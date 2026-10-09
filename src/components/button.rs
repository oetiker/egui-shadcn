//! shadcn Button: variants (default/destructive/outline/secondary/ghost/link)
//! x sizes (sm/default/lg/icon). Custom-painted for full per-state control.

use crate::components::icon::{paint_icon, Icon};
use crate::components::shared::{corner, focus_ring, hover_fill, mix_toward};
use crate::Theme;
use egui::{Color32, Response, Sense, Stroke, StrokeKind, Ui, Vec2, Widget};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Sm,
    Default,
    Lg,
    Icon,
}

pub struct Button {
    text: String,
    variant: ButtonVariant,
    size: ButtonSize,
    enabled: bool,
    icon: Option<&'static Icon>,
}

impl Button {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), variant: ButtonVariant::Default, size: ButtonSize::Default, enabled: true, icon: None }
    }
    pub fn variant(mut self, v: ButtonVariant) -> Self {
        self.variant = v;
        self
    }
    pub fn size(mut self, s: ButtonSize) -> Self {
        self.size = s;
        self
    }
    /// Disabled buttons do not sense clicks, cannot take focus, and paint
    /// halfway toward the page background (shadcn `disabled:opacity-50`).
    pub fn enabled(mut self, yes: bool) -> Self {
        self.enabled = yes;
        self
    }
    /// A leading icon (shadcn `<Button><Icon />Text</Button>`). With
    /// `ButtonSize::Icon` only the icon is drawn and the text is the button's
    /// accessible name.
    pub fn icon(mut self, icon: &'static Icon) -> Self {
        self.icon = Some(icon);
        self
    }
}

struct Colors {
    fill: Color32,
    text: Color32,
    border: Option<Color32>,
}

fn colors_for(variant: ButtonVariant, t: &Theme) -> Colors {
    let p = &t.palette;
    match variant {
        ButtonVariant::Default => Colors { fill: p.primary, text: p.primary_foreground, border: None },
        ButtonVariant::Destructive => Colors { fill: p.destructive, text: p.destructive_foreground, border: None },
        ButtonVariant::Secondary => Colors { fill: p.secondary, text: p.secondary_foreground, border: None },
        ButtonVariant::Outline => Colors { fill: p.background, text: p.foreground, border: Some(p.border) },
        ButtonVariant::Ghost => Colors { fill: Color32::TRANSPARENT, text: p.foreground, border: None },
        ButtonVariant::Link => Colors { fill: Color32::TRANSPARENT, text: p.primary, border: None },
    }
}

/// Colors of a hovered button. Ghost and Outline swap to the `accent` pair
/// (shadcn `hover:bg-accent hover:text-accent-foreground`): the text must
/// follow the fill, or a palette whose `accent` is saturated leaves light text
/// on a light fill. Link has no background on hover (text-decoration only);
/// the solid variants lighten toward the page background.
fn hovered_colors(variant: ButtonVariant, c: Colors, t: &Theme) -> Colors {
    let p = &t.palette;
    match variant {
        ButtonVariant::Ghost | ButtonVariant::Outline => {
            Colors { fill: p.accent, text: p.accent_foreground, border: c.border }
        }
        ButtonVariant::Link => Colors { fill: Color32::TRANSPARENT, ..c },
        _ => Colors { fill: hover_fill(c.fill, p.background), ..c },
    }
}

impl Widget for Button {
    fn ui(self, ui: &mut Ui) -> Response {
        let t = Theme::current(ui.ctx());
        let m = &t.metrics;
        let (height, pad_x) = match self.size {
            ButtonSize::Sm => (m.control_sm, m.pad_sm),
            ButtonSize::Default => (m.control_md, m.pad_md),
            ButtonSize::Lg => (m.control_lg, m.pad_lg),
            ButtonSize::Icon => (m.control_md, 0.0),
        };
        let mut c = colors_for(self.variant, &t);
        if !self.enabled {
            let bg = t.palette.background;
            c.fill = mix_toward(c.fill, bg, 0.5);
            c.text = mix_toward(c.text, bg, 0.5);
            c.border = c.border.map(|b| mix_toward(b, bg, 0.5));
        }

        // Lay out the text with the explicit color + medium family baked into the
        // galley, so it always wins over the global `override_text_color`
        // (shadcn buttons are font-medium).
        let fam = crate::theme::family(ui.ctx(), crate::theme::FAMILY_MEDIUM);
        let galley = ui.painter().layout_no_wrap(
            self.text.clone(),
            egui::FontId::new(m.text_sm, fam.clone()),
            c.text,
        );

        let icon_only = self.size == ButtonSize::Icon && self.icon.is_some();
        let icon_w = if self.icon.is_some() { m.icon } else { 0.0 };
        let content_w = if icon_only {
            icon_w
        } else if self.icon.is_some() {
            icon_w + m.gap + galley.size().x
        } else {
            galley.size().x
        };
        let width = if self.size == ButtonSize::Icon { height } else { content_w + pad_x * 2.0 };
        let desired = Vec2::new(width, height);
        let sense = if self.enabled { Sense::click() } else { Sense::hover() };
        let (rect, resp) = ui.allocate_exact_size(desired, sense);
        resp.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, self.enabled && ui.is_enabled(), &self.text)
        });

        let hovered = resp.hovered();
        if self.enabled && hovered {
            c = hovered_colors(self.variant, c, &t);
        }
        let fill = c.fill;

        let corner = corner(t.radius_md());
        if ui.is_rect_visible(rect) {
            if fill != Color32::TRANSPARENT {
                ui.painter().rect_filled(rect, corner, fill);
            }
            if let Some(b) = c.border {
                ui.painter().rect_stroke(rect, corner, Stroke::new(m.border, b), StrokeKind::Inside);
            }
            let mut x = rect.center().x - content_w / 2.0;
            if let Some(icon) = self.icon {
                let icon_rect = egui::Rect::from_min_size(
                    egui::pos2(x, rect.center().y - icon_w / 2.0),
                    Vec2::splat(icon_w),
                );
                paint_icon(ui.painter(), icon_rect, icon, c.text, m.icon_stroke);
                x += icon_w + m.gap;
            }
            if !icon_only {
                let pos = egui::pos2(x, rect.center().y - galley.size().y / 2.0);
                ui.painter().galley(pos, galley, c.text);
            }
        }
        focus_ring(ui, &resp, &t, t.radius_md());
        resp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ghost_and_outline_hover_switch_text_with_fill() {
        // A palette whose accent is as light as its foreground (the dark
        // theme's primary pair), so `foreground` text on it would vanish: the
        // text must become `accent_foreground`.
        let mut t = Theme::dark();
        t.palette.accent = t.palette.primary;
        t.palette.accent_foreground = t.palette.primary_foreground;
        for v in [ButtonVariant::Ghost, ButtonVariant::Outline] {
            let h = hovered_colors(v, colors_for(v, &t), &t);
            assert_eq!(h.fill, t.palette.accent);
            assert_eq!(h.text, t.palette.accent_foreground);
        }
        // Outline keeps its border on hover.
        let h = hovered_colors(ButtonVariant::Outline, colors_for(ButtonVariant::Outline, &t), &t);
        assert_eq!(h.border, Some(t.palette.border));
    }

    #[test]
    fn solid_hover_keeps_text() {
        let t = Theme::dark();
        let c = colors_for(ButtonVariant::Default, &t);
        let h = hovered_colors(ButtonVariant::Default, colors_for(ButtonVariant::Default, &t), &t);
        assert_eq!(h.text, c.text);
        assert_ne!(h.fill, c.fill);
    }
}
