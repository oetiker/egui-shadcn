//! Token discipline: components take every color and text size from the
//! theme, so restyling means editing `theme.rs` alone.

use egui_shadcn::components::button::Button;
use egui_shadcn::components::input::Input;
use egui_shadcn::{Metrics, Theme};

/// Color constructors and literal font sizes. A line may opt out with a
/// `// token-ok: <reason>` comment on the line above.
fn violations(source: &str) -> Vec<String> {
    let patterns = [
        "Color32::from_",
        "Color32::WHITE",
        "Color32::BLACK",
        "Color32::GRAY",
        "Color32::RED",
        "from_black_alpha",
        "from_white_alpha",
    ];
    let mut out = Vec::new();
    let mut prev = "";
    for (n, line) in source.lines().enumerate() {
        let literal_size = ["FontId::new(", "FontId::proportional(", "FontId::monospace(", ".size("]
            .iter()
            .any(|p| line.split(p).skip(1).any(|rest| rest.starts_with(|c: char| c.is_ascii_digit())));
        let literal_color = patterns.iter().any(|p| line.contains(p));
        if (literal_size || literal_color) && !prev.contains("token-ok:") {
            out.push(format!("{}: {}", n + 1, line.trim()));
        }
        prev = line;
    }
    out
}

#[test]
fn scanner_catches_literals() {
    // Positive control: the scanner must flag each kind it claims to.
    assert_eq!(violations("let c = Color32::from_rgb(1, 2, 3);").len(), 1);
    assert_eq!(violations("let c = Color32::WHITE;").len(), 1);
    assert_eq!(violations("let f = FontId::proportional(12.0);").len(), 1);
    assert_eq!(violations("RichText::new(t).size(14.0)").len(), 1);
    assert_eq!(violations("// token-ok: test\nlet c = Color32::WHITE;").len(), 0);
    assert_eq!(violations("let f = FontId::proportional(t.metrics.text_xs);").len(), 0);
}

#[test]
fn components_use_theme_tokens() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/src");
    let mut files: Vec<_> = std::fs::read_dir(format!("{dir}/components"))
        .expect("components dir")
        .map(|e| e.expect("dir entry").path())
        .collect();
    files.push(format!("{dir}/layout.rs").into());
    assert!(files.len() > 5, "scanned too few files: {files:?}");
    let mut found = Vec::new();
    for f in files {
        let src = std::fs::read_to_string(&f).expect("readable source");
        for v in violations(&src) {
            found.push(format!("{}:{v}", f.display()));
        }
    }
    assert!(found.is_empty(), "literal colors/sizes outside theme.rs:\n{}", found.join("\n"));
}

#[test]
fn metrics_drive_component_sizes() {
    let theme = Theme { metrics: Metrics { control_md: 44.0, ..Metrics::default() }, ..Theme::dark() };
    let ctx = egui::Context::default();
    let mut heights = (0.0, 0.0);
    let mut text = String::new();
    for _ in 0..2 {
        let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
            theme.apply(ui.ctx());
            heights.0 = ui.add(Button::new("Save")).rect.height();
            heights.1 = ui.add(Input::new(&mut text)).rect.height();
        });
    }
    assert_eq!(heights, (44.0, 44.0));
}
