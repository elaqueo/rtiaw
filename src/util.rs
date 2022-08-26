use cgmath::Vector3;

pub type Vec3 = Vector3<f32>;

pub fn length(v: &Vec3) -> f32 {
    (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}

pub fn square_length(v: &Vec3) -> f32 {
    v.x * v.x + v.y * v.y + v.z * v.z
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / length(&v)
}
