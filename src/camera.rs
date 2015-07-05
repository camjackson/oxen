use transform::Transform;

pub struct Camera {
    pub transform: Transform,
}

impl Camera {
    pub fn view_transform(&self) -> [[f32; 4]; 4] {
        self.transform.as_view_matrix()
    }
}
