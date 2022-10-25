//! Optimization algorithms for overlap detection, n-body distances, etc.
//!
//! Could possibly use the Parry crate for collision detection algorithms?

use crate::particle::{Particle, ParticleReference};

//---------------------------------------------------------------------------------------------------//

pub trait CollisionDetector {
    fn gather(&mut self, particles: &[Particle]) -> Vec<ParticleReference>;
}

//---------------------------------------------------------------------------------------------------//
