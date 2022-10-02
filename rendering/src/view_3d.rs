use engine::vec3::Vec3;

//---------------------------------------------------------------------------------------------------//

#[derive(Copy, Clone)]
pub struct View3D {
    pub camera_pos: Vec3,
    pub view_plane_pos: Vec3,
    pub fov: f64,
}

impl View3D {
    pub fn new() -> View3D {
        View3D {
            camera_pos: Vec3::zero(),
            view_plane_pos: Vec3::new(0.0, 0.0, -1.0),
            fov: 170.0 * std::f64::consts::PI / 180.0,
        }
    }
}
