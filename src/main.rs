use ray::Ray;
use util::*;

mod ray;
mod util;

fn color(r: &Ray) -> Vec3 {
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t)
        * Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
        + t * Vec3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        }
}

fn main() {
    let nx = 200;
    let ny = 100;

    print!("P3\n{} {}\n255\n", nx, ny);

    let lower_left_corner: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.0, 0.0);
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u: f32 = i as f32 / nx as f32;
            let v: f32 = j as f32 / ny as f32;

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&r);

            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
