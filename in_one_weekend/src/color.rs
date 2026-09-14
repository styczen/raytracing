use crate::{Interval, Vec3};
use std::io::{Result, Write};

pub type Color = Vec3;

#[inline(always)]
fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(out: &mut impl Write, pixel_color: Color) -> Result<()> {
    let r = linear_to_gamma(pixel_color.x);
    let g = linear_to_gamma(pixel_color.y);
    let b = linear_to_gamma(pixel_color.z);

    // Translate the [0,1] component values to the byte range [0,255].
    const INTENSITY: Interval = Interval::new(0.000, 0.999);
    let rbyte = (256.0 * INTENSITY.clamp(r)) as u8;
    let gbyte = (256.0 * INTENSITY.clamp(g)) as u8;
    let bbyte = (256.0 * INTENSITY.clamp(b)) as u8;
    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)
}
