pub use crate::particle::{Particle, ParticleReference};
pub use crate::vec3::Vec3;

//---------------------------------------------------------------------------------------------------//

pub struct ConstraintData<const PARTICLE_COUNT: usize> {
    pub constrained_particles: [ParticleReference; PARTICLE_COUNT],
    pub compliance: f64,
    pub dissipation: f64,

    constraint_function: Box<dyn Fn(&[Particle; PARTICLE_COUNT]) -> f64>,
    constraint_gradient: Box<dyn Fn(&Particle) -> Vec3>,
}

impl<const PARTICLE_COUNT: usize> ConstraintData<PARTICLE_COUNT> {
    fn project(&self, particles: &mut [Particle], dt: f64) {}
}

//---------------------------------------------------------------------------------------------------//

pub trait Constraint<const PARTICLE_COUNT: usize> {
    fn data(&mut self) -> &mut ConstraintData<PARTICLE_COUNT>;
}

//---------------------------------------------------------------------------------------------------//

impl<const PARTICLE_COUNT: usize> dyn Constraint<PARTICLE_COUNT> {
    pub fn handle(&mut self, _particles: &mut [Particle], _dt: f64) {}
}

//---------------------------------------------------------------------------------------------------//
