use std::io::{BufWriter, stdout};

use vec3::Vec3;
use ray::{Point3, Ray};

mod vec3;
mod ray;

const ASPECT_RATIO: f64 = 16f64 / 9f64;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

fn main() {
    let viewport_height = 2f64;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1f64;

    let origin = Point3::default();
    let horizontal = Vec3::with_val(viewport_width, 0f64, 0f64);
    let vertical = Vec3::with_val(0f64, viewport_height, 0f64);
    let lower_left_corner = origin - horizontal / 2f64 - vertical / 2f64 - Vec3::with_val(0f64, 0f64, focal_length);

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let output = stdout();
    let mut handle = BufWriter::new(output.lock());
    for height in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", height);
        for width in 0..IMAGE_WIDTH {
            let u = width as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = height as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::with_val(&origin, &(lower_left_corner + u * horizontal + v * vertical - origin));
            let color = r.ray_color();

            color.write_color(&mut handle).unwrap();
        }
    }

    eprintln!("Done.")
}
