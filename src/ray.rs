use cgmath::Vector3;

pub struct Ray {
    a: Vector3<f32>,
    b: Vector3<f32>,
}

impl Ray {
    pub fn new(a: Vector3<f32>, b: Vector3<f32>) -> Self {
        Self { a, b }
    }

    pub fn origin(&self) -> &Vector3<f32> {
        &self.a
    }

    pub fn direction(&self) -> &Vector3<f32> {
        &self.b
    }

    pub fn point_at(&self, t: f32) -> Vector3<f32> {
        self.a + t * self.b
    }
}
