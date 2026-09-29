use rand::{SeedableRng, rngs::StdRng};
use raytracing_in_one_weekend::{
    Camera, CameraConfig, Color, Dielectric, HittableList, Lambertian, Metal, Point3, Sphere, Vec3,
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
    let material_left = Dielectric {
        refraction_index: 1.5,
    };
    let material_bubble = Dielectric {
        refraction_index: 1.00 / 1.5,
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
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        Rc::new(material_bubble),
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_right),
    ));

    // let r = std::f64::consts::FRAC_PI_4.cos();
    //
    // let material_left = Lambertian {
    //     albedo: Color::new(0.0, 0.0, 1.0),
    // };
    // let material_right = Lambertian {
    //     albedo: Color::new(1.0, 0.0, 0.0),
    // };
    //
    // world.add(Sphere::new(
    //     Point3::new(-r, 0.0, -1.0),
    //     r,
    //     Rc::new(material_left),
    // ));
    // world.add(Sphere::new(
    //     Point3::new(r, 0.0, -1.0),
    //     r,
    //     Rc::new(material_right),
    // ));

    let camera = Camera::new(CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 20.0,
        lookfrom: Point3::new(-2.0, 2.0, 1.0),
        lookat: Point3::new(0.0, 0.0, -1.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 10.0,
        focus_dist: 3.4,
    });

    let mut rng = StdRng::seed_from_u64(0);
    camera.render(&world, &mut rng)?;

    Ok(())
}
