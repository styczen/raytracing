use std::io::{self, BufWriter, Write};

use raytracing_in_one_weekend::{Color, write_color};

fn main() -> io::Result<()> {
    let width: usize = 256;
    let height: usize = 256;

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    writeln!(out, "P3")?;
    writeln!(out, "{} {}", width, height)?;
    writeln!(out, "255")?;

    for i in 0..height {
        eprint!("\rScanlines remaining: {} ", height - i);
        for j in 0..width {
            let r: f64 = i as f64 / (width - 1) as f64;
            let g: f64 = j as f64 / (height - 1) as f64;
            let b: f64 = 0.0;

            let pixel_color = Color::new(r, g, b);
            write_color(&mut out, pixel_color)?
        }
    }

    out.flush()?;
    eprintln!("Done");
    Ok(())
}
