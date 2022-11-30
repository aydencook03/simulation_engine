use crate::{
    interaction::{pair_wise::PairWiseForce, Interaction},
    math::{Point3, Vec3, PI},
    particle::{Particle, ParticleReference},
};

//---------------------------------------------------------------------------------------------------//

pub struct SphParameters {
    coupled_particles: Vec<ParticleReference>,
    kernel: Box<dyn SphKernel>,
    constitutive_relation: Option<Box<dyn Constitutive>>,
    smoothing_radius: Option<f64>,
}

impl SphParameters {
    pub fn density(&self, point: Point3, neighbors: &[Particle]) -> f64 {
        todo!()
    }

    pub fn interpolate<T>(
        &self,
        point: Point3,
        neighbors: &[Particle],
        retrieve: impl Fn(&Particle) -> T,
    ) -> T {
        todo!()
    }

    pub fn one(&self, point: Point3, neighbors: &[Particle]) -> f64 {
        todo!()
    }

    pub fn zero(&self, point: Point3, neighbors: &[Particle]) -> f64 {
        todo!()
    }

    pub fn pair_force(&self, particle1: &Particle, particle2: &Particle) -> Vec3 {
        todo!()
    }
}

pub trait SphKernel {
    fn w(&self, r: Point3, h: f64) -> f64;
    fn grad_w(&self, r: Point3, h: f64) -> Vec3;
    fn div_w(&self, r: Point3, h: f64) -> f64;
    fn laplace_w(&self, r: Point3, h: f64) -> f64;
}

pub trait Constitutive {
    fn pressure(&self, particle: &Particle, density: f64) -> f64;
}

impl Interaction for SphParameters {
    fn handle(&mut self, _particle_source: &mut [Particle], _dt: f64) {
        todo!()
    }
}

impl PairWiseForce for SphParameters {
    fn force(&self, particle1: &Particle, particle2: &Particle) -> Option<Vec3> {
        todo!()
    }
}

//---------------------------------------------------------------------------------------------------//
// Builtin Kernels

pub struct GaussianKernel(f64);

impl SphKernel for GaussianKernel {
    fn w(&self, r: Point3, h: f64) -> f64 {
        (h.powi(3) * PI.powi(3).sqrt()).powi(-1) * (-r.mag_squared() / h.powi(2)).exp()
    }

    fn grad_w(&self, r: Point3, h: f64) -> Vec3 {
        -2. * (h.powi(5) * PI.powi(3).sqrt()).powi(-1) * (-r.mag_squared() / h.powi(2)).exp() * r
    }

    fn div_w(&self, r: Point3, h: f64) -> f64 {
        todo!()
    }

    fn laplace_w(&self, r: Point3, h: f64) -> f64 {
        todo!()
    }
}

//---------------------------------------------------------------------------------------------------//
