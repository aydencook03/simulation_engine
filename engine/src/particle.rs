pub use crate::vec3::Vec3;

//---------------------------------------------------------------------------------------------------//

#[derive(Default)]
pub struct Particle {
    // identity
    pub id: u32,
    pub group: u32,

    // properties
    pub mass: f64,
    pub charge: f64,
    pub temperature: f64,
    pub radius: f64,

    // state
    pub pos: Vec3,
    pub vel: Vec3,
    pub prev_pos: Vec3,
    pub time_since_prev_pos: f64,

    // dynamical influences
    pub forces: Vec<Vec3>,
    pub impulses: Vec<Vec3>,
    pub displacements: Vec<Vec3>,
    // pub internal_work: Vec<f64>,
    // pub in_contact_with: Vec<ParticleReference>,
    // pub collision_info: CollisionInfo, restitution, friction, in_contact_with, thermal diffusivity, etc
    // pub temperature_info: TempInfo, // conduction & expansion constants, temp_sim: bool, etc
}

//---------------------------------------------------------------------------------------------------//
// Particle associated functions and methods.

impl Particle {
    pub fn new() -> Particle {
        Particle::default().mass(10.0).radius(10.0)
    }

    pub fn id(mut self, id: u32) -> Particle {
        self.id = id;
        self
    }

    pub fn mass(mut self, mass: f64) -> Particle {
        self.mass = mass;
        self
    }

    pub fn radius(mut self, radius: f64) -> Particle {
        self.radius = radius;
        self
    }

    pub fn pos(mut self, pos: Vec3) -> Particle {
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

    //--------------------------------------------------------------------//

    pub fn inverse_mass(&self) -> f64 {
        if self.mass > 0_f64 {
            1_f64 / self.mass
        } else {
            0_f64
        }
    }

    pub fn integrate(&mut self, dt: f64) {
        // Scaling by inverse mass ensures that dynamical interactions conserve momentum and center of mass.
        // An inverse mass of 0 (infinitely massive) then allows for particles that don't experience
        // interactions (ie: barriers, etc).
        let inverse_mass = self.inverse_mass();

        let mut total_force = Vec3::zero();
        for force in &self.forces {
            total_force += *force;
        }

        let mut total_impulse = Vec3::zero();
        for impulse in &self.impulses {
            total_impulse += *impulse;
        }

        let mut total_displacement = Vec3::zero();
        for displacement in &self.displacements {
            total_displacement += *displacement;
        }

        self.vel += total_force * inverse_mass * dt;
        self.vel += total_impulse * inverse_mass;
        self.impulses.clear();

        self.prev_pos = self.pos;
        self.time_since_prev_pos = dt;

        self.pos += self.vel * dt;
        self.pos += total_displacement * self.inverse_mass();
        self.displacements.clear();
    }

    pub fn clear_force(&mut self) {
        self.forces.clear();
    }

    pub fn vel_from_prev_pos(&mut self) {
        self.vel = (self.pos - self.prev_pos) / self.time_since_prev_pos;
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
    pub fn get_mut(mut self, list: &mut [Particle]) -> &mut Particle {
        if list[self.index].id == self.id {
            return &mut list[self.index];
        } else {
            for (index, particle) in list.into_iter().enumerate() {
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
            for (index, particle) in list.into_iter().enumerate() {
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
