use engine::math::{Point3, Vec3};

//---------------------------------------------------------------------------------------------------//

#[derive(Copy, Clone, Default)]
pub struct Camera3D {
    pub pos: Point3,
    pub dir: Vec3,
    focal_length: f64,

    init_pos: Point3,
    init_dir: Vec3,
    init_focal_length: f64,
}

impl Camera3D {
    pub fn new() -> Camera3D {
        Camera3D::default()
            .pos(0.0, 0.0, 1500.0)
            .dir(0.0, 0.0, -1.0)
            .focal_length(1500.0)
    }

    pub fn pos(mut self, x: f64, y: f64, z: f64) -> Camera3D {
        self.pos = Point3::new(x, y, z);
        self.init_pos = Point3::new(x, y, z);
        self
    }

    pub fn dir(mut self, x: f64, y: f64, z: f64) -> Camera3D {
        self.dir = Vec3::new(x, y, z).norm();
        self.init_dir = Vec3::new(x, y, z);
        self
    }

    pub fn focal_length(mut self, focal_length: f64) -> Camera3D {
        self.focal_length = focal_length;
        self.init_focal_length = focal_length;
        self
    }

    pub fn focus_on_point(&mut self, point: Vec3) {
        self.dir = (point - self.pos).norm();
    }

    pub fn reset(&mut self) {
        self.pos = self.init_pos;
        self.dir = self.init_dir;
        self.focal_length = self.init_focal_length;
    }

    pub fn dist_to_cam(&self, point: Point3) -> f64 {
        (point - self.pos).dot(self.dir)
    }

    pub fn perspective_point(&self, point: Point3) -> Point3 {
        let p = point;
        let c = self.pos;
        let f = self.focal_length;
        let n = self.dir;

        ((p - c - f * n).mag_squared() - ((p - c).dot(n) - f).powi(2) + f.powi(2)).sqrt()
            * (p - c).norm()
            - f * n
    }

    pub fn perspective_sphere(&self, point: Point3, radius: f64) -> (Point3, f64) {
        let mapped_point = self.perspective_point(point);
        let scaled_radius =
            ((self.perspective_point(point + mapped_point.norm() * radius)) - mapped_point).mag();

        println!("{:#?}, {:#?}", mapped_point, scaled_radius);

        (mapped_point, scaled_radius)
    }
}

//---------------------------------------------------------------------------------------------------//
