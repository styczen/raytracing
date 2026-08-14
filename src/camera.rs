use rand::prelude::*;
use std::io::{BufWriter, Write};

use crate::{Color, Hittable, Interval, Point3, Ray, Vec3, write_color};

pub struct Camera {
    // aspect_ratio: f64,
    image_width: usize,
    image_height: usize,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: usize,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn new(image_width: usize, aspect_ratio: f64, samples_per_pixel: usize) -> Self {
        // Image
        let image_height = ((image_width as f64 / aspect_ratio) as usize).max(1);

        let center = Point3::default();

        // Determine viewport dimensions
        let focal_legth = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate vectors across horizontal and vertcal edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate delta vectors
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate location of the upper left pixel
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_legth) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        Self {
            // aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
        }
    }

    pub fn render(&self, world: &impl Hittable) -> std::io::Result<()> {
        let stdout = std::io::stdout();
        let mut out = BufWriter::new(stdout.lock());

        writeln!(out, "P3")?;
        writeln!(out, "{} {}", self.image_width, self.image_height)?;
        writeln!(out, "255")?;

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(r, world);
                }
                // let pixel_center = self.pixel00_loc
                //     + i as f64 * self.pixel_delta_u
                //     + j as f64 * self.pixel_delta_v;
                // let ray_direction = pixel_center - self.center;
                // let r = Ray::new(self.center, ray_direction);
                //
                // let pixel_color = Camera::ray_color(r, world);
                write_color(&mut out, self.pixel_samples_scale * pixel_color)?
            }
        }

        out.flush()?;
        Ok(())
    }

    // Construct a camera ray originating from the origin and directed at randmly
    // samples point aroung the pixel location.
    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = Vec3::new(
            rand::random_range(0.0..=1.0) - 0.5,
            rand::random_range(0.0..=1.0) - 0.5,
            0.0,
        );
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn ray_color(r: Ray, world: &impl Hittable) -> Color {
        if let Some(hit_record) = world.hit(r, Interval::new(0.0, f64::INFINITY)) {
            0.5 * (hit_record.normal + Color::new(1.0, 1.0, 1.0))
        } else {
            let unit_direction = r.dir.unit_vector();
            let a = 0.5 * (unit_direction.y + 1.0);
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
    }
}
