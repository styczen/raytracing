use rand::{SeedableRng, rngs::StdRng};
use raytracing_in_one_weekend::{
    Camera, CameraConfig, Color, HittableList, Lambertian, Metal, Point3, Sphere,
};
use std::rc::Rc;

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();

    let material_ground = Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let material_center = Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };
    let material_left = Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
        fuzz: 0.3,
    };
    let material_right = Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(material_ground),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        Rc::new(material_center),
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_left),
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_right),
    ));

    let camera = Camera::new(CameraConfig {
        samples_per_pixel: 100,
        max_depth: 50,
        ..Default::default()
    });

    let mut rng = StdRng::seed_from_u64(0);
    camera.render(&world, &mut rng)?;

    Ok(())
}
