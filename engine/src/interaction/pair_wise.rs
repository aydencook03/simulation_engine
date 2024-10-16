use crate::{
    interaction::Interaction,
    math::Vec3,
    particle::{Particle, ParticleReference},
};

//---------------------------------------------------------------------------------------------------//

pub struct PairWiseForceParameters {
    coupled_particles: Vec<ParticleReference>,
    force: Box<dyn PairWiseForce>,
}

pub trait PairWiseForce {
    fn force(&self, particle1: &Particle, particle2: &Particle) -> Option<Vec3>;
}

impl PairWiseForceParameters {
    pub fn new(force: impl PairWiseForce + 'static) -> PairWiseForceParameters {
        PairWiseForceParameters {
            coupled_particles: Vec::new(),
            force: Box::new(force),
        }
    }

    pub fn with_particle(mut self, reference: ParticleReference) -> PairWiseForceParameters {
        self.coupled_particles.push(reference);
        self
    }

    pub fn with_particles(mut self, references: &[ParticleReference]) -> PairWiseForceParameters {
        for reference in references {
            self.coupled_particles.push(*reference);
        }
        self
    }
}

impl Interaction for PairWiseForceParameters {
    fn handle(&mut self, particle_source: &mut [Particle], _dt: f64) {
        for (index, ref1) in self.coupled_particles.iter().enumerate() {
            for ref2 in &self.coupled_particles[(index + 1)..] {
                if let Some(force) = self
                    .force
                    .force(ref1.get(particle_source), ref2.get(particle_source))
                {
                    let p1 = ref1.get_mut(particle_source);
                    p1.add_force(force);
                    let p2 = ref2.get_mut(particle_source);
                    p2.add_force(-force);
                }
            }
        }
    }
}

//---------------------------------------------------------------------------------------------------//
