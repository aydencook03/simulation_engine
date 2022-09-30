//! An object that enables dynamic interaction with and between particles.
//! A field can also store its own state and have an integration method.

use crate::particle::{Particle, ParticleReference};
use crate::vec3::Vec3;

//---------------------------------------------------------------------------------------------------//

#[derive(Default, Copy, Clone)]
pub struct ParticleAction {
    force: Option<Vec3>,
    impulse: Option<Vec3>,
    displacement: Option<Vec3>,
    internal_work: Option<f64>,
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
    pub fn work(mut self, work: f64) -> ParticleAction {
        self.internal_work = Some(work);
        self
    }
}

//---------------------------------------------------------------------------------------------------//

pub enum InteractionType {
    ParticleParticle,
    FieldParticle,
}

//---------------------------------------------------------------------------------------------------//

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
