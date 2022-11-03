pub struct Particle {
    // identity
    pub id: u32,
    pub group: u32,

    // force calculations
    pub mass: f64,
    pub charge: f64,

    // integration
    pub inverse_mass: f64,
    pub prev_pos: Point3,

    // state
    pub pos: Point3,
    pub vel: Vec3,
    pub temperature: f64,

    // optional
    pub forces: Option<Vec<Force>>,
    pub extent: Option<Extent>,
}

pub struct Force(pub Vec3, pub Option<Point3>);
impl Force {
    pub fn from_impulse(impulse: Vec3, dt: f64) -> Force {
        Force(impulse/dt, None)
    }

    pub fn from_displacement(displacement: Vec3, mass: f64, dt: f64) -> Force {
        Force(mass*displacement/dt.powi(2), None)
    }
}

pub struct Extent {
    pub inverse_inertia: Matrix3,
    pub prev_orientation: Vec3,

    pub orientation: Vec3,
    pub angular_velocity: Vec3,

    pub shape: Shape,
}