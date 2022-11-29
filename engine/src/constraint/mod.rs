pub mod builtin_constraints;
pub mod xpbd;

//---------------------------------------------------------------------------------------------------//
use crate::particle::Particle;

pub trait Constraint {
    fn project(&mut self, particle_source: &mut [Particle], dt: f64, static_pass: bool);
}

//---------------------------------------------------------------------------------------------------//