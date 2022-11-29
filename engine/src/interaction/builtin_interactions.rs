use crate::{
    interaction::{
        pair_wise::{PairWiseForce, PairWiseForceParameters},
        simple::{SimpleForce, SimpleForceParameters},
    },
    math::Vec3,
    particle::Particle,
};

//---------------------------------------------------------------------------------------------------//

pub struct ConstantForce(Vec3);

impl ConstantForce {
    pub fn new(force: Vec3) -> SimpleForceParameters {
        SimpleForceParameters::new(ConstantForce(force))
    }
}

impl SimpleForce for ConstantForce {
    fn force(&self, _particle: &Particle) -> Option<Vec3> {
        Some(self.0)
    }
}

//--------------------------------------------------------------------//

pub struct Falling(f64);

impl Falling {
    pub fn new(g: f64) -> SimpleForceParameters {
        SimpleForceParameters::new(Falling(g))
    }
}

impl SimpleForce for Falling {
    fn force(&self, particle: &Particle) -> Option<Vec3> {
        Some(Vec3::new(0.0, -particle.mass * self.0, 0.0))
    }
}

//--------------------------------------------------------------------//

pub struct Gravity(f64);

impl Gravity {
    pub fn new(gravitational_constant: f64) -> PairWiseForceParameters {
        PairWiseForceParameters::new(Gravity(gravitational_constant))
    }
}

impl PairWiseForce for Gravity {
    fn force(&self, particle1: &Particle, particle2: &Particle) -> Option<Vec3> {
        let radial = particle2.pos - particle1.pos;
        let dist = radial.mag();

        Some(((self.0 * particle1.mass * particle2.mass) / dist.powi(3)) * radial)
    }
}

//--------------------------------------------------------------------//

pub struct ElectroStatic(f64);

impl ElectroStatic {
    pub fn new(electrostatic_constant: f64) -> PairWiseForceParameters {
        PairWiseForceParameters::new(ElectroStatic(electrostatic_constant))
    }
}

impl PairWiseForce for ElectroStatic {
    fn force(&self, particle1: &Particle, particle2: &Particle) -> Option<Vec3> {
        let radial = particle2.pos - particle1.pos;
        let dist = radial.mag();

        Some(-((self.0 * particle1.charge * particle2.charge) / dist.powi(3)) * radial)
    }
}

//--------------------------------------------------------------------//

pub struct LennardJones {
    dispersion_energy: f64,
    collision_radius: f64,
    repulsion: f64,
    attraction: f64,
}

impl LennardJones {
    pub fn new(dispersion_energy: f64, collision_radius: f64) -> LennardJones {
        LennardJones {
            dispersion_energy,
            collision_radius,
            repulsion: 12.,
            attraction: 6.,
        }
    }

    pub fn mie_potential(mut self, repulsion: f64, attraction: f64) -> LennardJones {
        self.repulsion = repulsion;
        self.attraction = attraction;
        self
    }

    pub fn build(self) -> PairWiseForceParameters {
        PairWiseForceParameters::new(self)
    }
}

impl PairWiseForce for LennardJones {
    fn force(&self, particle1: &Particle, particle2: &Particle) -> Option<Vec3> {
        let (n, m) = (self.repulsion, self.attraction);
        let c = (n / (n - m)) * ((n / m).powf(m / (n - m))) * self.dispersion_energy;
        let radial = particle2.pos - particle1.pos;
        let sigma = self.collision_radius;
        let r = radial.mag();

        Some(
            -c * ((n * sigma.powf(n) / r.powf(n + 2.)) - (m * sigma.powf(m) / r.powf(m + 2.)))
                * radial,
        )
    }
}

//---------------------------------------------------------------------------------------------------//