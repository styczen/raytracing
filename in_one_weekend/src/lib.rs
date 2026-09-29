mod vec3;
pub use vec3::{Point3, Vec3};

mod color;
pub use color::{Color, write_color};

mod ray;
pub use ray::Ray;

mod hittable;
pub use hittable::{HitRecord, Hittable};

mod sphere;
pub use sphere::Sphere;

mod hittable_list;
pub use hittable_list::HittableList;

mod interval;
pub use interval::Interval;

mod camera;
pub use camera::{Camera, CameraConfig};

mod material;
pub use material::{Material, Lambertian, Metal, Dielectric};
