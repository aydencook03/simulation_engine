pub use crate::math::{Matrix3, Point3, Vec3};

//---------------------------------------------------------------------------------------------------//

#[derive(Default)]
pub struct Particle {
    // identity
    pub id: u32,
    pub group: u32,

    // properties
    pub mass: f64,
    pub charge: f64,
    pub extra: u128,

    // dynamics
    pub inverse_mass: f64,
    pub prev_pos: Point3,
    pub forces: Vec<Force>,

    // state
    pub pos: Point3,
    pub vel: Vec3,
    pub temperature: f64,
    pub extent: Option<Extent>,
}

//--------------------------------------------------------------------//

#[derive(Copy, Clone, Default)]
pub struct Force(pub Vec3, pub Option<Point3>);
impl Force {
    pub fn from_impulse(impulse: Vec3, dt: f64) -> Force {
        Force(impulse / dt, None)
    }

    pub fn from_displacement(displacement: Vec3, mass: f64, dt: f64) -> Force {
        Force(mass * displacement / dt.powi(2), None)
    }

    pub fn location(mut self, location: Point3) -> Force {
        self.1 = Some(location);
        self
    }
}
impl core::ops::Neg for Force {
    type Output = Force;
    fn neg(self) -> Self::Output {
        Force(-self.0, self.1)
    }
}

//--------------------------------------------------------------------//

#[derive(Copy, Clone)]
pub struct Extent {
    pub inverse_inertia: Matrix3,
    pub prev_orientation: Vec3,

    pub orientation: Vec3,
    pub angular_velocity: Vec3,

    pub shape: Shape,
}

#[derive(Copy, Clone)]
pub enum Shape {
    Sphere,
    Cuboid,
    Capsule,
    Cylinder,
    Cone,
    Compound,
    ConvexMesh,
    TriangleMesh,
    Heightfield,
}

//---------------------------------------------------------------------------------------------------//
// Particle associated functions and methods.

impl Particle {
    pub fn new() -> Particle {
        Particle::default().mass(10.0)
    }

    // builder methods for particle identity
    pub fn id(mut self, id: u32) -> Particle {
        self.id = id;
        self
    }
    pub fn group(mut self, group: u32) -> Particle {
        self.group = group;
        self
    }

    // builder methods for particle properties
    pub fn force_mass(mut self, mass: f64) -> Particle {
        self.mass = mass;
        self
    }
    pub fn charge(mut self, charge: f64) -> Particle {
        self.charge = charge;
        self
    }
    pub fn inverse_mass(mut self, inverse_mass: f64) -> Particle {
        self.inverse_mass = inverse_mass;
        self
    }
    pub fn mass(mut self, mass: f64) -> Particle {
        self.mass = mass;
        self.inverse_mass = if mass != 0.0 { 1.0 / mass } else { 0.0 };
        self
    }

    // builder methods for particle state
    pub fn pos(mut self, pos: Point3) -> Particle {
        self.pos = pos;
        self
    }
    pub fn pos_xyz(mut self, x: f64, y: f64, z: f64) -> Particle {
        self.pos.x = x;
        self.pos.y = y;
        self.pos.z = z;
        self
    }
    pub fn vel(mut self, vel: Vec3) -> Particle {
        self.vel = vel;
        self
    }
    pub fn vel_xyz(mut self, vel_x: f64, vel_y: f64, vel_z: f64) -> Particle {
        self.vel.x = vel_x;
        self.vel.y = vel_y;
        self.vel.z = vel_z;
        self
    }
    pub fn temperature(mut self, temp: f64) -> Particle {
        self.temperature = temp;
        self
    }

    //--------------------------------------------------------------------//
    // physics methods

    pub fn integrate(&mut self, dt: f64) {
        let mut total_force = Vec3::zero();

        for force in &self.forces {
            total_force += force.0;
        }

        self.vel += total_force * self.inverse_mass * dt;
        self.prev_pos = self.pos;
        self.pos += self.vel * dt;
    }

    pub fn update_vel(&mut self, dt: f64) {
        self.vel = (self.pos - self.prev_pos) / dt;
    }
}

//---------------------------------------------------------------------------------------------------//
// ParticleReference struct with associated functions and methods.

/// A lightweight reference to a particle that obeys Rust's rules.
#[derive(Copy, Clone)]
pub struct ParticleReference {
    pub id: u32,
    pub index: usize,
}

impl ParticleReference {
    pub fn new(id: u32, index: usize) -> ParticleReference {
        ParticleReference { id, index }
    }

    pub fn get_mut(mut self, list: &mut [Particle]) -> &mut Particle {
        if list[self.index].id == self.id {
            return &mut list[self.index];
        } else {
            for (index, particle) in list.iter_mut().enumerate() {
                if particle.id == self.id {
                    self.index = index;
                    return particle;
                }
            }
        }
        panic!();
    }

    pub fn get(mut self, list: &[Particle]) -> &Particle {
        if list[self.index].id == self.id {
            return &list[self.index];
        } else {
            for (index, particle) in list.iter().enumerate() {
                if particle.id == self.id {
                    self.index = index;
                    return particle;
                }
            }
        }
        panic!();
    }
}

//---------------------------------------------------------------------------------------------------//
