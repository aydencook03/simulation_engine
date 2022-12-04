use crate::{
    interaction::{pair_wise::PairWiseForce, Interaction},
    math::{Point3, Vec3, PI},
    particle::Particle,
};

//---------------------------------------------------------------------------------------------------//

pub struct SphParameters {
    kernel: Box<dyn SphKernel>,
    state_equation: Option<Box<dyn StateEquation>>,
    smoothing_radius: Option<f64>,
    compact: bool,
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

    pub fn pressure_force(&self, particle1: &Particle, particle2: &Particle) -> Vec3 {
        todo!()
    }
}

pub trait SphKernel {
    fn w(&self, r: Point3, h: f64) -> f64;
    fn grad_w(&self, r: Point3, h: f64) -> Vec3;
    fn div_w(&self, r: Point3, h: f64) -> f64;
    fn laplace_w(&self, r: Point3, h: f64) -> f64;
}

pub trait StateEquation {
    fn pressure(&self, particle: &Particle) -> f64;
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

pub struct GaussianKernel;
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

pub struct Poly6Kernel;
impl SphKernel for Poly6Kernel {
    fn w(&self, r: Point3, h: f64) -> f64 {
        315. * (64. * PI * h.powi(9)).powi(-1) * (h.powi(2) - r.mag_squared()).powi(3)
    }

    fn grad_w(&self, r: Point3, h: f64) -> Vec3 {
        todo!()
    }

    fn div_w(&self, r: Point3, h: f64) -> f64 {
        todo!()
    }

    fn laplace_w(&self, r: Point3, h: f64) -> f64 {
        todo!()
    }
}

pub struct SpikyKernel;
impl SphKernel for SpikyKernel {
    fn w(&self, r: Point3, h: f64) -> f64 {
        //15. * (PI * h.powi(6)).powi(-1) * (h - r.mag()).powi(3)
        todo!()
    }

    fn grad_w(&self, r: Point3, h: f64) -> Vec3 {
        let mag = r.mag();
        -45. * (PI * h.powi(6)).powi(-1) * (h - mag).powi(2) * mag.powi(-1) * r
    }

    fn div_w(&self, r: Point3, h: f64) -> f64 {
        todo!()
    }

    fn laplace_w(&self, r: Point3, h: f64) -> f64 {
        todo!()
    }
}

pub struct ViscosityKernel;
impl SphKernel for ViscosityKernel {
    fn w(&self, r: Point3, h: f64) -> f64 {
        todo!()
    }

    fn grad_w(&self, r: Point3, h: f64) -> Vec3 {
        todo!()
    }

    fn div_w(&self, r: Point3, h: f64) -> f64 {
        todo!()
    }

    fn laplace_w(&self, r: Point3, h: f64) -> f64 {
        45. * (PI * h.powi(6)).powi(-1) * (h - r.mag())
    }
}

//---------------------------------------------------------------------------------------------------//
