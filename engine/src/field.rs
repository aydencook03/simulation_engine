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

    pub fn send_to_particle(&self, particle: &mut Particle) {
        if let Some(force) = self.force {
            particle.forces.push(force);
        }
        if let Some(impulse) = self.impulse {
            particle.impulses.push(impulse);
        }
        if let Some(displacement) = self.displacement {
            particle.displacements.push(displacement);
        }
    }
}

//---------------------------------------------------------------------------------------------------//

#[derive(Copy, Clone)]
pub enum InteractionType {
    FieldParticle,
    ParticleParticle,
}

//---------------------------------------------------------------------------------------------------//

pub trait Field {
    fn coupled_particles(&self) -> &[ParticleReference];
    fn interaction_type(&self) -> InteractionType;
    fn is_constraint(&self) -> bool { false }
    fn particle_to_field(&mut self, _particle: &Particle) {}
    fn integrate(&mut self, _dt: f64) {}
    fn field_to_particle(&self, particle: &Particle) -> ParticleAction;
    fn clear(&mut self) {}
}

impl dyn Field {
    pub fn handle_fields(fields: &mut [Box<dyn Field>], particles: &mut [Particle], dt: f64) {
        let mut non_constraint_fields = Vec::new();

        for field in fields {
            if field.is_constraint() {
                field.handle(particles, dt);
            } else {
                non_constraint_fields.push(field);
            }
        }

        for field in non_constraint_fields {
            field.handle(particles, dt);
        }
    }

    pub fn handle(&mut self, particles: &mut [Particle], dt: f64) {
        match self.interaction_type() {
            InteractionType::FieldParticle => {
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
                    action.send_to_particle(particle);
                }

                self.clear();
            },
            InteractionType::ParticleParticle => (),
        }
    }
}

//---------------------------------------------------------------------------------------------------//
