use egui_shadcn::color::{oklch_to_srgb, oklch_to_srgb_a};

#[test]
fn white_is_white() {
    let c = oklch_to_srgb(1.0, 0.0, 0.0);
    assert_eq!((c.r(), c.g(), c.b()), (255, 255, 255));
}

#[test]
fn achromatic_grays_match_zinc() {
    // (lightness, expected gray) verified against shadcn v4 zinc ramp.
    let cases = [
        (0.985, 250u8),
        (0.922, 229),
        (0.708, 161),
        (0.556, 115),
        (0.269, 38),
        (0.205, 23),
        (0.145, 10),
    ];
    for (l, expected) in cases {
        let c = oklch_to_srgb(l, 0.0, 0.0);
        let got = c.r();
        assert!(
            (got as i16 - expected as i16).abs() <= 1,
            "L={l}: expected ~{expected}, got {got}"
        );
        assert_eq!(c.r(), c.g(), "L={l} not gray");
        assert_eq!(c.g(), c.b(), "L={l} not gray");
    }
}

#[test]
fn destructive_red_is_reddish() {
    let c = oklch_to_srgb(0.577, 0.245, 27.325);
    assert!(c.r() > 200, "expected strong red, got r={}", c.r());
    assert!(c.g() < 90 && c.b() < 90, "expected low g/b, got {:?}", (c.g(), c.b()));
}

#[test]
fn alpha_is_passed_through() {
    let c = oklch_to_srgb_a(1.0, 0.0, 0.0, 0.10);
    assert_eq!(c.a(), 26); // round(0.10 * 255)
}
