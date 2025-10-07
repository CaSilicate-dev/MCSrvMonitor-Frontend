pub fn percent_color(
    (r1, g1, b1): (u8, u8, u8),
    (r2, g2, b2): (u8, u8, u8),
    percentage: f64,
) -> String {
    let p = percentage.clamp(0.0, 1.0);

    let r = r1 as f64 + (r2 as f64 - r1 as f64) * p;
    let g = g1 as f64 + (g2 as f64 - g1 as f64) * p;
    let b = b1 as f64 + (b2 as f64 - b1 as f64) * p;

    format!("{:02x}{:02x}{:02x}", r.round() as u8, g.round() as u8, b.round() as u8)
}
