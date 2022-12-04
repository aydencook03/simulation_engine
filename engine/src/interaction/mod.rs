pub mod field;
pub mod interactions;
pub mod pair_wise;
pub mod simple;
pub mod sph;

//---------------------------------------------------------------------------------------------------//
use crate::particle::Particle;

pub trait Interaction {
    fn handle(&mut self, particle_source: &mut [Particle], dt: f64);
}

//---------------------------------------------------------------------------------------------------//
