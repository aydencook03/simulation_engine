use crate::{
    interaction::Interaction,
    math::Vec3,
    particle::{Particle, ParticleReference},
};

//---------------------------------------------------------------------------------------------------//

pub struct SimpleForceParameters {
    coupled_particles: Vec<ParticleReference>,
    force: Box<dyn SimpleForce>,
}

pub trait SimpleForce {
    fn force(&self, particle: &Particle) -> Option<Vec3>;
}

impl SimpleForceParameters {
    pub fn new(force: impl SimpleForce + 'static) -> SimpleForceParameters {
        SimpleForceParameters {
            coupled_particles: Vec::new(),
            force: Box::new(force),
        }
    }

    pub fn with_particle(mut self, reference: ParticleReference) -> SimpleForceParameters {
        self.coupled_particles.push(reference);
        self
    }

    pub fn with_particles(mut self, references: &[ParticleReference]) -> SimpleForceParameters {
        for reference in references {
            self.coupled_particles.push(*reference);
        }
        self
    }
}

impl Interaction for SimpleForceParameters {
    fn handle(&mut self, particle_source: &mut [Particle], _dt: f64) {
        for reference in &self.coupled_particles {
            if let Some(force) = self.force.force(reference.get(particle_source)) {
                let particle = reference.get_mut(particle_source);
                particle.add_force(force, particle.pos);
            }
        }
    }
}

//---------------------------------------------------------------------------------------------------//