//! An object that enables dynamic interaction with and between particles. 
//! A field can also have its own state and integration method.

use crate::particle::{Particle, ParticleReference};
use crate::vec3::Vec3;

//---------------------------------------------------------------------------------------------------//

#[derive(Default)]
pub struct ParticleAction {
    force: Option<Vec3>,
    impulse: Option<Vec3>,
    displacement: Option<Vec3>,
    // internal_work
}

impl ParticleAction {
    pub fn new() -> ParticleAction {
        ParticleAction::default()
    }
    pub fn force(mut self, force: Vec3) -> ParticleAction {
        self.force = Some(force);
        self
    }
    pub fn impulse(mut self, impulse: Vec3) -> ParticleAction {
        self.impulse = Some(impulse);
        self
    }
    pub fn displacement(mut self, displacement: Vec3) -> ParticleAction {
        self.displacement = Some(displacement);
        self
    }
}

pub trait Field {
    fn coupled_particles(&self) -> &[ParticleReference];
    fn particle_to_field(&mut self, _particle: &Particle) {}
    fn integrate(&mut self, _dt: f64) {}
    fn field_to_particle(&self, particle: &Particle) -> ParticleAction;
    fn clear(&mut self) {}
}

impl dyn Field {
    pub fn handle(&mut self, particles: &mut [Particle], dt: f64) {
        // particles -> act on field
        for reference in self.coupled_particles().to_owned() {
            // need to optimize
            self.particle_to_field(reference.get(particles));
        }

        // field dynamics
        self.integrate(dt);

        // field -> act on particles
        for reference in self.coupled_particles() {
            let particle = reference.get_mut(particles);
            let action = self.field_to_particle(particle);
            if let Some(force) = action.force {
                particle.forces.push(force);
            }
            if let Some(impulse) = action.impulse {
                particle.impulses.push(impulse);
            }
            if let Some(displacement) = action.displacement {
                particle.displacements.push(displacement);
            }
        }

        self.clear();
    }
}

//---------------------------------------------------------------------------------------------------//

pub struct Gravity(Vec<ParticleReference>, f64);
impl Gravity {
    pub fn new(g: f64) -> Gravity {
        Gravity(Vec::new(), g)
    }

    pub fn add_particle(&mut self, particle_reference: ParticleReference) {
        self.0.push(particle_reference);
    }
}
impl Field for Gravity {
    fn coupled_particles(&self) -> &[ParticleReference] {
        &self.0
    }
    fn field_to_particle(&self, particle: &Particle) -> ParticleAction {
        ParticleAction::new().force(Vec3::new(0.0, 0.0, -particle.mass * self.1))
    }
}

pub struct NGravity(Vec<ParticleReference>, Vec<(f64, Vec3)>, f64);
impl NGravity {
    pub fn new(gravitational_constant: f64) -> NGravity {
        NGravity(Vec::new(), Vec::new(), gravitational_constant)
    }

    pub fn add_particle(&mut self, particle_reference: ParticleReference) {
        self.0.push(particle_reference);
    }
}
impl Field for NGravity {
    fn coupled_particles(&self) -> &[ParticleReference] {
        &self.0
    }
    fn particle_to_field(&mut self, particle: &Particle) {
        self.1.push((particle.mass, particle.pos));
    }
    fn field_to_particle(&self, particle: &Particle) -> ParticleAction {
        let mut total_force = Vec3::zero();
        for attractor in &self.1 {
            let radial = particle.pos - attractor.1;

            if radial.mag_squared() > 0_f64 {
                total_force +=
                    radial * -(self.2 * attractor.0 * particle.mass) / radial.mag().powi(3);
            }
        }

        ParticleAction::new().force(total_force)
    }
    fn clear(&mut self) {
        self.1.clear();
    }
}
