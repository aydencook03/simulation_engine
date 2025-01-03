use crate::math::{Point3, Vec3};

//---------------------------------------------------------------------------------------------------//

#[derive(Default)]
pub struct Particle {
    // identity
    pub id: u32,
    pub group: u32, // should this be turned into a collection of groups?

    // state
    pub pos: Point3,
    pub vel: Vec3,

    // dynamics
    pub mass: f64,
    pub inverse_mass: f64,
    pub prev_pos: Point3,
    pub forces: Vec<Vec3>,

    // continuum sampling
    pub density: f64,
    pub temperature: f64,
    pub entropy: f64,

    // electromagnetic properties
    pub charge: f64,
}

//---------------------------------------------------------------------------------------------------//
// Particle associated functions and methods.

impl Particle {
    pub fn new() -> Particle {
        Particle::default().mass(10.0)
    }

    //--------------------------------------------------------------------//
    // builder methods for particle identity

    pub fn id(mut self, id: u32) -> Particle {
        self.id = id;
        self
    }
    pub fn group(mut self, group: u32) -> Particle {
        self.group = group;
        self
    }

    //--------------------------------------------------------------------//
    // builder methods for particle dynamics properties

    pub fn force_mass(mut self, mass: f64) -> Particle {
        self.mass = mass;
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

    //--------------------------------------------------------------------//
    // builder methods for particle state

    pub fn pos(mut self, pos: Point3) -> Particle {
        self.pos = pos;
        self
    }
    pub fn pos_xyz(mut self, x: f64, y: f64, z: f64) -> Particle {
        self.pos = Point3::new(x, y, z);
        self
    }
    pub fn vel(mut self, vel: Vec3) -> Particle {
        self.vel = vel;
        self
    }
    pub fn vel_xyz(mut self, vel_x: f64, vel_y: f64, vel_z: f64) -> Particle {
        self.vel = Vec3::new(vel_x, vel_y, vel_z);
        self
    }

    //--------------------------------------------------------------------//
    // physics methods

    pub fn integrate(&mut self, dt: f64) {
        let mut total_force = Vec3::zero();

        for force in &self.forces {
            total_force += *force;
        }

        self.vel += total_force * self.inverse_mass * dt;
        self.prev_pos = self.pos;
        self.pos += self.vel * dt;
    }

    pub fn add_force(&mut self, force: Vec3) {
        self.forces.push(force);
    }

    pub fn add_displacement(
        &mut self,
        displacement: Vec3,
        _at_point: Point3,
        as_force: bool,
        dt: f64,
    ) {
        if !as_force {
            self.pos += displacement;
        } else {
            self.forces.push(self.mass * displacement / dt.powi(2));
        }
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
