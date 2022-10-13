pub use crate::particle::{Particle, ParticleReference};
pub use crate::vec3::Vec3;

//---------------------------------------------------------------------------------------------------//

pub enum ConstrainedParticles {
    None,
    One(ParticleReference),
    Two([ParticleReference; 2]),
    Three([ParticleReference; 2]),
    Four([ParticleReference; 4]),
    N(Vec<ParticleReference>),
}

//---------------------------------------------------------------------------------------------------//

pub struct ParticleCorrection(Option<Vec3>);

impl ParticleCorrection {
    pub fn new() -> ParticleCorrection {
        ParticleCorrection(None)
    }

    pub fn correction(mut self, correction: Vec3) -> ParticleCorrection {
        self.0 = Some(correction);
        self
    }

    pub fn project(&self, particle: &mut Particle) {
        if let Some(correction) = self.0 {
            particle.pos += correction;
            // or should I do particle.displacements.push(correction)?
        }
    }
}

//---------------------------------------------------------------------------------------------------//

pub trait Constraint {
    fn constrained_particles(&self) -> &ConstrainedParticles;
    fn is_broken(&self) -> bool {
        false
    }
    fn calculate_corrections(&self, particles: &[Particle], dt: f64) -> Option<Vec<ParticleCorrection>>;
}

//---------------------------------------------------------------------------------------------------//

impl dyn Constraint {
    pub fn handle(&mut self, particles: &mut [Particle], dt: f64) {
        if !self.is_broken() {
            
        }
    }
}

//---------------------------------------------------------------------------------------------------//