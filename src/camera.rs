use transform::Transform;

pub struct Camera {
    pub transform: Transform,
}

impl Camera {
    pub fn view_transform(&self) -> [[f32; 4]; 4] {
        self.transform.as_view_matrix()
    }

    pub fn perspective_transform(&self) -> [[f32; 4]; 4] {
        let aspect_ratio = self.transform.scale_ratio();
        let fov: f32 = 3.141592 / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f / aspect_ratio, 0.0,              0.0              ,   0.0],
            [       0.0      ,  f ,              0.0              ,   0.0],
            [       0.0      , 0.0,  (zfar+znear)/(znear-zfar)    ,  -1.0],
            [       0.0      , 0.0,  (2.0*zfar*znear)/(znear-zfar),   0.0],
        ]
    }
}
