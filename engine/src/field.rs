//! An object that enables dynamic interaction with and between particles.
//! A field can also store its own state and have an integration method.

use crate::particle::{Particle, ParticleReference};
use crate::vec3::Vec3;

//---------------------------------------------------------------------------------------------------//

#[derive(Default, Clone)]
pub struct CoupledParticles(pub Vec<ParticleReference>);

impl CoupledParticles {
    pub fn new() -> CoupledParticles {
        CoupledParticles::default()
    }
}

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

    pub fn immediate_constraint_action(&self, particle: &mut Particle) {
        if let Some(displacement) = self.displacement {
            particle.pos += displacement;
        }
        if let Some(impulse) = self.impulse {
            particle.vel += impulse * particle.inverse_mass();
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
    fn coupled_particles(&self) -> &CoupledParticles;
    fn coupled_particles_mut(&mut self) -> &mut CoupledParticles;
    fn add_particle(&mut self, particle_reference: ParticleReference) {
        self.coupled_particles_mut().0.push(particle_reference);
    }
    fn add_particles(&mut self, particle_references: &[ParticleReference]) {
        for reference in particle_references {
            self.coupled_particles_mut().0.push(*reference);
        }
    }
    fn interaction_type(&self) -> InteractionType;
    fn is_constraint(&self) -> bool {
        false
    }
    fn particle_to_field(&mut self, _particle: &Particle) {}
    fn integrate(&mut self, _dt: f64) {}
    fn field_to_particle(&self, _particle: &Particle) -> ParticleAction {
        ParticleAction::new()
    }
    fn clear(&mut self) {}
    fn particle_to_particle(&self, _particle1: &Particle, _particle2: &Particle) -> ParticleAction {
        ParticleAction::new()
    }
}

//--------------------------------------------------------------------//

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
                for reference in &self.coupled_particles().0.to_owned() {
                    // need to find a way around the ".to_owned()"
                    self.particle_to_field(reference.get(particles));
                }

                // field dynamics
                self.integrate(dt);

                // field -> act on particles
                for reference in &self.coupled_particles().0 {
                    let particle = reference.get_mut(particles);
                    let action = self.field_to_particle(particle);
                    if self.is_constraint() {
                        action.immediate_constraint_action(particle);
                    } else {
                        action.send_to_particle(particle);
                    }
                }

                self.clear();
            }
            InteractionType::ParticleParticle => {
                for ref1 in &self.coupled_particles().0 {
                    for ref2 in &self.coupled_particles().0 {
                        if ref1.id != ref2.id {
                            let action =
                                self.particle_to_particle(ref1.get(particles), ref2.get(particles));

                            let particle1 = ref1.get_mut(particles);
                            if self.is_constraint() {
                                action.immediate_constraint_action(particle1);
                            } else {
                                action.send_to_particle(particle1);
                            }
                        }
                    }
                }
            }
        }
    }
}

//---------------------------------------------------------------------------------------------------//
