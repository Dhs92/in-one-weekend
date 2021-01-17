#![allow(dead_code)]
use crate::vec3::Vec3;
pub(crate) type Point3 = Vec3;
type Color = Vec3;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3
}

impl Ray {
    pub fn new() -> Self {
        Self {
            origin: Point3::default(),
            direction: Vec3::default(),
        }
    }

    pub fn with_val(origin: &Point3, direction: &Vec3) -> Self {
        Self {
            origin: *origin,
            direction: *direction
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn ray_color(&self) -> Color {
        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1f64);

        (1f64 - t) * Color::with_val(1f64, 1f64, 1f64) + t * Color::with_val(0.5, 0.7, 1f64)
    }
}