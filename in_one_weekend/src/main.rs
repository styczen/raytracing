use rand::{SeedableRng, rngs::StdRng};
use raytracing_in_one_weekend::{Camera, CameraConfig, HittableList, Point3, Sphere};

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    let camera = Camera::new(CameraConfig {
        samples_per_pixel: 100,
        max_depth: 50,
        ..Default::default()
    });

    let mut rng = StdRng::seed_from_u64(0);
    camera.render(&world, &mut rng)?;

    Ok(())
}
