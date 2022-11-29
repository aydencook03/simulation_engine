use crate::{
    interaction::Interaction,
    math::{Point3, Vec3},
    particle::ParticleReference,
};

//---------------------------------------------------------------------------------------------------//

pub struct SphParameters {
    _coupled_particles: Vec<ParticleReference>,
    _kernel: Box<dyn SphKernel>,
    _smoothing_radius: Option<f64>,
}

impl SphParameters {}

pub trait SphKernel {
    fn w(&self, r: Point3, h: f64) -> f64;
    fn grad_w(&self, r: Point3, h: f64) -> Vec3;
    fn div_w(&self, r: Point3, h: f64) -> f64;
    fn laplace_w(&self, r: Point3, h: f64) -> f64;
}

impl Interaction for SphParameters {
    fn handle(&mut self, _particle_source: &mut [crate::prelude::Particle], _dt: f64) {
        todo!()
    }
}
