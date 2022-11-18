use engine::math::Vec3;

//---------------------------------------------------------------------------------------------------//

#[derive(Copy, Clone, Default)]
pub struct Camera3D {
    pos: Vec3,
    dir: Vec3,
    focal_length: f64,

    init_pos: Vec3,
    init_dir: Vec3,
    init_focal_length: f64,
}

impl Camera3D {
    pub fn new() -> Camera3D {
        Camera3D::default().dir(0.0, 0.0, -1.0).focal_length(1500.0)
    }

    pub fn pos(mut self, x: f64, y: f64, z: f64) -> Camera3D {
        self.pos = Vec3::new(x, y, z);
        self.init_pos = Vec3::new(x, y, z);
        self
    }

    fn dir(mut self, x: f64, y: f64, z: f64) -> Camera3D {
        self.dir = Vec3::new(x, y, z).norm();
        self.init_dir = Vec3::new(x, y, z);
        self
    }

    fn focal_length(mut self, focal_length: f64) -> Camera3D {
        self.focal_length = focal_length;
        self.init_focal_length = focal_length;
        self
    }

    pub fn reset(&mut self) {
        self.pos = self.init_pos;
        self.dir = self.init_dir;
        self.focal_length = self.init_focal_length;
    }

    pub fn orthographic_sphere(&self, pos: Vec3, radius: f64) -> (Vec3, f64) {
        (pos, radius)
    }

    pub fn perspective_sphere(&self, pos: Vec3, radius: f64) -> (Vec3, f64) {
        (pos, radius)
    }
}

//---------------------------------------------------------------------------------------------------//
