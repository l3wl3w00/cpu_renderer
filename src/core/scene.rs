use crate::input::SceneAction;
use cgmath::{Array, ElementWise, InnerSpace, Vector3, Zero};
use std::cmp::Ordering;
use std::time::Duration;
use crate::core::camera::Camera;
use crate::core::common::{HitData, NormalizedVector3, Ray};
use crate::core::light::Light;
use crate::core::shapes::Sphere;

pub struct Scene {
    camera: Camera,
    spheres: Vec<Sphere>,
    lights: Vec<Light>,
    ambient_light_color: Vector3<f32>,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Vector3<f32> {
        let first_hit = if let Some(hit) = self.intersect(&ray) {
            hit
        } else {
            return self.ambient_light_color;
        };
        let mut out_color = self.ambient_light_color.mul_element_wise(first_hit.color);

        for light in &self.lights {
            let intersection_to_light = light.position - first_hit.intersection;
            let origin = first_hit.intersection + f32::EPSILON * first_hit.normal.get();
            let shadow_ray = Ray::new(origin, intersection_to_light);
            let first_shadow_hit = self.intersect(&shadow_ray);
            if first_shadow_hit.is_none() {
                continue;
            }

            let distance_to_light2 = intersection_to_light.magnitude2();
            let cos_theta = f32::max(intersection_to_light.normalize().dot(first_hit.normal.get()), 0.0);
            out_color += first_hit.color * cos_theta * 2. / distance_to_light2.sqrt() + Vector3::from_value(0.15);
        }
        out_color
    }
    pub fn intersect(&self, ray: &Ray) -> Option<HitData> {
        self.spheres
            .iter()
            .filter_map(|sphere| sphere.intersect(&ray))
            .min_by(|hit1, hit2| hit1.t
                .partial_cmp(&hit2.t)
                .unwrap_or(Ordering::Equal))
    }
    pub fn new(camera: Camera) -> Scene {
        Scene {
            camera,
            spheres: Vec::new(),
            lights: Vec::new(),
            ambient_light_color: Vector3::new(0.1, 0.1, 0.1),
        }
    }
    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere)
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn tick(&mut self, actions: impl Iterator<Item=SceneAction>, time_provider: &impl TimeProvider) {
        self.camera.tick(actions, time_provider.dt());

        let first_light_pos = if let Some(&mut ref mut light) = self.lights.first_mut() {
            &mut light.position
        } else {
            return;
        };

        let light_orbit_point = Vector3::new(0.0, 1.0, -6.0);
        let light_orbit_radius = 3.0;

        let elapsed = time_provider.total_time().as_secs_f32();
        let orbit_speed = 1.0;
        let angle = elapsed * orbit_speed;

        let new_light_x = light_orbit_point.x + light_orbit_radius * angle.cos();
        let new_light_z = light_orbit_point.z + light_orbit_radius * angle.sin();

        let new_light_position = Vector3::new(new_light_x, first_light_pos.y, new_light_z);

        *first_light_pos = new_light_position;

        self.spheres[0].set_center(*first_light_pos)
    }
}

pub trait TimeProvider {
    fn total_time(&self) -> &Duration;
    fn dt(&self) -> &Duration;
}