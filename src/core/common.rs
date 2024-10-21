use cgmath::num_traits::Float;
use cgmath::{InnerSpace, Vector3, VectorSpace};
pub struct HitData {
    pub intersection: Vector3<f32>,
    pub t: f32,
    pub normal: NormalizedVector3<f32>,
    pub color: Vector3<f32>,
}

pub struct NormalizedVector3<T>(Vector3<T>);

impl<T: Float> NormalizedVector3<T>
where Vector3<T>: InnerSpace,
      <Vector3<T> as VectorSpace>::Scalar: Float {
    pub fn from_vector3(v: Vector3<T>) -> Self {
        NormalizedVector3(v.normalize())
    }

    pub fn get(&self) -> Vector3<T> {self.0}
}

pub struct Ray {
    origin: Vector3<f32>,
    direction: NormalizedVector3<f32>
}

impl Ray {
    pub fn position_at(&self, t: f32) -> Vector3<f32> {
        self.origin + self.direction.0 * t
    }

    pub(super) fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin, direction: NormalizedVector3::from_vector3(direction)
        }
    }

    pub fn origin(&self) -> &Vector3<f32> { &self.origin }
    pub fn direction(&self) -> &NormalizedVector3<f32> { &self.direction }
}
