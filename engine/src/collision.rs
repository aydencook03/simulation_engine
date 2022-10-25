use crate::particle::{Particle, ParticleReference};

//---------------------------------------------------------------------------------------------------//

pub trait CollisionDetector {
    fn gather(&mut self, particles: &[Particle]) -> Vec<ParticleReference>;
}

//---------------------------------------------------------------------------------------------------//