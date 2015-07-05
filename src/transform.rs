use na::Vec3;

pub struct Transform {
    position: Vec3<f32>,
    rotation: Vec3<f32>,
    scale: Vec3<f32>,
    pub visible: bool,
}

impl Transform {
    pub fn new(position_x: f32, position_y: f32, position_z: f32,
               rotation_x: f32, rotation_y: f32, rotation_z: f32,
               scale_x: f32, scale_y: f32, scale_z: f32,
               visible: bool) -> Transform {
        Transform{
            position: Vec3::new(position_x, position_y, position_z),
            rotation: Vec3::new(rotation_x, rotation_y, rotation_z),
            scale: Vec3::new(scale_x, scale_y, scale_z),
            visible: visible,
        }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.position = self.position + Vec3{x: x, y: y, z: z};
    }

    pub fn as_matrix(&self) -> [[f32; 4]; 4] {
        [
            [self.scale.x, 0., 0., 0.],
            [0., self.scale.y, 0., 0.],
            [0., 0., self.scale.z, 0.],
            [self.position.x, self.position.y, self.position.z, 1.],
        ]
    }

    pub fn as_view_matrix(&self) -> [[f32; 4]; 4] {
        [
            [1. / self.scale.x, 0., 0., 0.],
            [0., 1. / self.scale.y, 0., 0.],
            [0., 0., 1. / self.scale.z, 0.],
            [-self.position.x, -self.position.y, -self.position.z, 1.],
        ]
    }
}
