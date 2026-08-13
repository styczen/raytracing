use raytracing_in_one_weekend::{Camera, HittableList, Point3, Sphere, Vec3};

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new(400, 16.0 / 9.0);

    camera.render(&world)?;

    Ok(())
}
