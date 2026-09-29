use rand::{Rng, RngExt};
use std::io::{BufWriter, Write};

use crate::{Color, Hittable, Interval, Point3, Ray, Vec3, write_color};

#[derive(Debug, Clone, Copy)]
pub struct CameraConfig {
    // Ratio of image width over height
    pub aspect_ratio: f64,
    // Rendered image width in pixel count
    pub image_width: usize,
    // Count of random samples for each pixel
    pub samples_per_pixel: usize,
    // Maximum number of ray bounces into scene
    pub max_depth: usize,
    // Vertical view angle (field of view)
    pub vfov: f64,
    // Point camera is looking from
    pub lookfrom: Point3,
    // Point camera is looking at
    pub lookat: Point3,
    // Camera-relative "up" direction
    pub vup: Vec3,
    // Variation angle of rays through each pixel
    pub defocus_angle: f64,
    // Distance from camera lookfrom point to plane of perfect focus
    pub focus_dist: f64,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            image_width: 400,
            aspect_ratio: 16.0 / 9.0,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::default(),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
        }
    }
}

pub struct Camera {
    image_width: usize,
    // Rendered image height
    image_height: usize,
    // Camera center
    center: Point3,
    // Location of pixel 0, 0
    pixel00_loc: Point3,
    // Offset to pixel to the right
    pixel_delta_u: Vec3,
    // Offset to pixel below
    pixel_delta_v: Vec3,
    samples_per_pixel: usize,
    // Color scale factor for a sum of pixel samples
    pixel_samples_scale: f64,
    max_depth: usize,
    // Camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_angle: f64,
    // Defocus disk horizontal radius
    defocus_disk_u: Vec3,
    // Defocus disk vertical radius
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(config: CameraConfig) -> Self {
        let CameraConfig {
            image_width,
            aspect_ratio,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
        } = config;

        // Image
        let image_height = ((image_width as f64 / aspect_ratio) as usize).max(1);

        let center = lookfrom;

        // Determine viewport dimensions
        // let focal_length = (lookfrom - lookat).length();
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        // Calculate vectors across horizontal and vertical edges
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate delta vectors
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate location of the upper left pixel
        let viewport_upper_left = center - focus_dist * w - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
            u,
            v,
            w,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &impl Hittable, rng: &mut impl Rng) -> std::io::Result<()> {
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
                    let r = self.get_ray(i, j, rng);
                    pixel_color += Self::ray_color(r, self.max_depth, world, rng);
                }
                write_color(&mut out, self.pixel_samples_scale * pixel_color)?;
            }
        }
        eprintln!();

        out.flush()?;
        Ok(())
    }

    /// Construct a camera ray originating from the defocus disk and directed at
    /// a randomly sampled point around the pixel location (i, j).
    fn get_ray(&self, i: usize, j: usize, rng: &mut impl Rng) -> Ray {
        let offset = Self::sample_square(rng);
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample(rng)
        };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    /// Returns a random point in the camera defocus disk.
    fn defocus_disk_sample(&self, rng: &mut impl Rng) -> Point3 {
        let p = Vec3::random_in_unit_disk(rng);
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    fn ray_color(r: Ray, depth: usize, world: &impl Hittable, rng: &mut impl Rng) -> Color {
        if depth == 0 {
            Color::new(0.0, 0.0, 0.0)
        } else {
            if let Some(hit_record) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
                // let direction = hit_record.normal + Vec3::random_unit_vector(rng);
                // 0.5 * Self::ray_color(Ray::new(hit_record.p, direction), depth - 1, world, rng)

                if let Some((attenuation, scattered)) = hit_record.mat.scatter(r, &hit_record, rng)
                {
                    attenuation * Self::ray_color(scattered, depth - 1, world, rng)
                } else {
                    Color::default()
                }
            } else {
                let unit_direction = r.dir.unit_vector();
                let a = 0.5 * (unit_direction.y + 1.0);
                (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
            }
        }
    }

    fn sample_square(rng: &mut impl Rng) -> Vec3 {
        Vec3::new(
            rng.random_range(-0.5..0.5),
            rng.random_range(-0.5..0.5),
            0.0,
        )
    }
}
