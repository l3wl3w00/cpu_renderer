use cgmath::{InnerSpace, Vector3};
use crate::core::common::{HitData, NormalizedVector3, Ray};

pub struct Sphere {
    center: Vector3<f32>,
    color: Vector3<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, color: Vector3<f32>) -> Sphere {
        Sphere { center, radius, color }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<HitData> {
        let oc = ray.origin() - self.center;
        let raydir = ray.direction().get();
        let a = raydir.dot(raydir);
        let b = 2.0 * oc.dot(raydir);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);

        // Choose the smallest positive t
        let t = if t1 > 0.0 && t2 > 0.0 {
            t1.min(t2)
        } else if t1 > 0.0 {
            t1
        } else if t2 > 0.0 {
            t2
        } else {
            return None; // Both intersections are behind the ray
        };

        let intersection = ray.position_at(t);
        Some(HitData{
            intersection,
            t,
            normal: self.normal(intersection),
            color: self.color,
        })
    }

    pub fn normal(&self, point: Vector3<f32>) -> NormalizedVector3<f32> {
        NormalizedVector3::from_vector3(point - self.center)
    }

    pub fn set_center(&mut self, point: Vector3<f32>) {
        self.center = point;
    }
}
