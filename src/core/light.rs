use cgmath::Vector3;

pub struct Light {
    pub position: Vector3<f32>
}

impl Light {
    pub fn from_position(position: Vector3<f32>) -> Light {
        Light { position }
    }
}