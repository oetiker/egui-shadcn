//! shadcn v4 (new-york / OKLCH) design tokens, materialized for egui.

use crate::color::{oklch_to_srgb as c, oklch_to_srgb_a as ca};
use egui::Color32;

/// The semantic shadcn color tokens. Surfaces come in `x` / `x_foreground` pairs.
#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub background: Color32,
    pub foreground: Color32,
    pub card: Color32,
    pub card_foreground: Color32,
    pub popover: Color32,
    pub popover_foreground: Color32,
    pub primary: Color32,
    pub primary_foreground: Color32,
    pub secondary: Color32,
    pub secondary_foreground: Color32,
    pub muted: Color32,
    pub muted_foreground: Color32,
    pub accent: Color32,
    pub accent_foreground: Color32,
    pub destructive: Color32,
    pub destructive_foreground: Color32,
    pub border: Color32,
    pub input: Color32,
    pub ring: Color32,
}

impl Palette {
    pub fn light() -> Self {
        Self {
            background: c(1.0, 0.0, 0.0),
            foreground: c(0.145, 0.0, 0.0),
            card: c(1.0, 0.0, 0.0),
            card_foreground: c(0.145, 0.0, 0.0),
            popover: c(1.0, 0.0, 0.0),
            popover_foreground: c(0.145, 0.0, 0.0),
            primary: c(0.205, 0.0, 0.0),
            primary_foreground: c(0.985, 0.0, 0.0),
            secondary: c(0.97, 0.0, 0.0),
            secondary_foreground: c(0.205, 0.0, 0.0),
            muted: c(0.97, 0.0, 0.0),
            muted_foreground: c(0.556, 0.0, 0.0),
            accent: c(0.97, 0.0, 0.0),
            accent_foreground: c(0.205, 0.0, 0.0),
            destructive: c(0.577, 0.245, 27.325),
            destructive_foreground: c(0.985, 0.0, 0.0),
            border: c(0.922, 0.0, 0.0),
            input: c(0.922, 0.0, 0.0),
            ring: c(0.708, 0.0, 0.0),
        }
    }

    pub fn dark() -> Self {
        Self {
            background: c(0.145, 0.0, 0.0),
            foreground: c(0.985, 0.0, 0.0),
            card: c(0.205, 0.0, 0.0),
            card_foreground: c(0.985, 0.0, 0.0),
            popover: c(0.205, 0.0, 0.0),
            popover_foreground: c(0.985, 0.0, 0.0),
            primary: c(0.922, 0.0, 0.0),
            primary_foreground: c(0.205, 0.0, 0.0),
            secondary: c(0.269, 0.0, 0.0),
            secondary_foreground: c(0.985, 0.0, 0.0),
            muted: c(0.269, 0.0, 0.0),
            muted_foreground: c(0.708, 0.0, 0.0),
            accent: c(0.269, 0.0, 0.0),
            accent_foreground: c(0.985, 0.0, 0.0),
            destructive: c(0.704, 0.191, 22.216),
            destructive_foreground: c(0.985, 0.0, 0.0),
            border: ca(1.0, 0.0, 0.0, 0.10),
            input: ca(1.0, 0.0, 0.0, 0.15),
            ring: c(0.556, 0.0, 0.0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub palette: Palette,
    pub radius: f32,
    pub dark: bool,
}

impl Theme {
    pub fn light() -> Self {
        Self { palette: Palette::light(), radius: 10.0, dark: false }
    }
    pub fn dark() -> Self {
        Self { palette: Palette::dark(), radius: 10.0, dark: true }
    }

    pub fn radius_sm(&self) -> f32 { self.radius - 4.0 }
    pub fn radius_md(&self) -> f32 { self.radius - 2.0 }
    pub fn radius_lg(&self) -> f32 { self.radius }
    pub fn radius_xl(&self) -> f32 { self.radius + 4.0 }
}
