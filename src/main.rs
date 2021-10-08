mod ray;
mod vec3;

use ray::Ray;
use std::fs::File;
use std::io::{BufWriter, Write};
use vec3::Vec3;

fn hit_sphere(center: &Vec3, radius:f64, ray: &Ray) -> bool {
    let oc: Vec3 = ray.origin() - *center;
    let a : f64 = ray.direction().dot(ray.direction());
    let b: f64 = 2.0* oc.dot(ray.direction());
    let c = oc.dot(oc) - radius*radius;
    let discriminant: f64 = b*b - 4.0*a*c;
    discriminant > 0.0
}

fn color(r: &Ray) -> Vec3 {
    let center = Vec3::new(0.0,0.0,-1.0);
    if hit_sphere(&center,0.5,&r) {
        return Vec3::new(1.0,0.0,0.0);
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0).scalar_mult(1.0 - t) + Vec3::new(0.5, 0.7, 1.0).scalar_mult(t)
}

fn main() -> std::io::Result<()> {
    let mut f = File::create("test.ppm")?;
    let nx: i32 = 200;
    let ny: i32 = 100;
    let s = format!("P3\n{} {}\n{}\n", nx, ny, 255);
    let buffer = BufWriter::new(f);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    buffer.write(s.as_bytes());
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u: f64 = i as f64 / nx as f64;
            let v: f64 = j as f64 / ny as f64;
            let r: Ray = Ray::new(origin, lower_left_corner + horizontal.scalar_mult(u) + vertical.scalar_mult(v));
            let vec = color(&r);
            let ir = (255.99 * vec[0]) as i32;
            let ig = (255.99 * vec[1]) as i32;
            let ib = (255.99 * vec[2]) as i32;
            let s = format!("{} {} {}\n", ir, ig, ib);
            buffer.write(s.as_bytes()); // need a buffered writer.
        }
    }

    let e1 = Vec3::new(2.0, 3.0, 5.0);
    let e2 = Vec3::new(2.0, 3.0, 5.0);
    println!("{:?}", e1 + e2);

    Ok(())
}
