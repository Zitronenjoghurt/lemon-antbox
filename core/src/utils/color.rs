pub fn interpolate_colors(a: [u8; 4], b: [u8; 4], t: f32) -> [u8; 4] {
    [
        (a[0] as f32 * (1.0 - t) + b[0] as f32 * t) as u8,
        (a[1] as f32 * (1.0 - t) + b[1] as f32 * t) as u8,
        (a[2] as f32 * (1.0 - t) + b[2] as f32 * t) as u8,
        (a[3] as f32 * (1.0 - t) + b[3] as f32 * t) as u8,
    ]
}

pub fn alpha_blend(a: [u8; 4], b: [u8; 4]) -> [u8; 4] {
    let alpha_a = a[3] as f32 / 255.0;
    let alpha_b = b[3] as f32 / 255.0;

    let alpha_out = alpha_a + alpha_b * (1.0 - alpha_a);

    if alpha_out == 0.0 {
        return [0, 0, 0, 0];
    }

    [
        ((a[0] as f32 * alpha_a + b[0] as f32 * alpha_b * (1.0 - alpha_a)) / alpha_out) as u8,
        ((a[1] as f32 * alpha_a + b[1] as f32 * alpha_b * (1.0 - alpha_a)) / alpha_out) as u8,
        ((a[2] as f32 * alpha_a + b[2] as f32 * alpha_b * (1.0 - alpha_a)) / alpha_out) as u8,
        (alpha_out * 255.0) as u8,
    ]
}
