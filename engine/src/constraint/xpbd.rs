use crate::{
    constraint::Constraint,
    math::Vec3,
    particle::{Particle, ParticleReference},
};

//---------------------------------------------------------------------------------------------------//

pub struct XpbdParameters {
    xpbd: Box<dyn Xpbd>,

    compliance: f64,
    dissipation: f64,

    force: f64,
    max_force: Option<f64>,
    broken: bool,

    as_force: bool,
    as_inequality: bool,
}

pub trait Xpbd {
    fn particles(&self) -> &[ParticleReference];
    fn constraint(&self, particles: &[&Particle]) -> f64;
    fn gradients(&self, particles: &[&Particle]) -> Vec<Vec3>;
}

impl XpbdParameters {
    pub fn new(xpbd: impl Xpbd + 'static) -> XpbdParameters {
        XpbdParameters {
            xpbd: Box::new(xpbd),
            compliance: 0.0,
            dissipation: 0.0,
            force: 0.0,
            max_force: None,
            broken: false,
            as_force: false,
            as_inequality: false,
        }
    }

    pub fn compliance(mut self, compliance: f64) -> XpbdParameters {
        self.compliance = compliance;
        self
    }

    pub fn dissipation(mut self, dissipation: f64) -> XpbdParameters {
        self.dissipation = dissipation;
        self
    }

    pub fn max_force(mut self, max_force: f64) -> XpbdParameters {
        self.max_force = Some(max_force);
        self
    }

    pub fn as_force(mut self) -> XpbdParameters {
        self.as_force = true;
        self
    }

    pub fn as_inequality(mut self) -> XpbdParameters {
        self.as_inequality = true;
        self
    }

    pub fn force_estimate(&self) -> f64 {
        self.force
    }
}

impl Constraint for XpbdParameters {
    fn project(&mut self, particle_source: &mut [Particle], dt: f64, static_pass: bool) {
        let (breakable, max_force) = match self.max_force {
            Some(force) => (true, force),
            None => (false, 0.0),
        };

        if !breakable || !self.broken {
            let particles: Vec<&Particle> = self
                .xpbd
                .particles()
                .iter()
                .map(|p| p.get(particle_source))
                .collect();

            let evaluated = self.xpbd.constraint(&particles);

            let satisfied = match self.as_inequality {
                false => evaluated == 0.0,
                true => evaluated >= 0.0,
            };

            if !satisfied {
                let dt = if static_pass { core::f64::MAX } else { dt };
                let alpha = self.compliance / dt.powi(2);
                let gamma = self.compliance * self.dissipation / dt;
                let gradients = self.xpbd.gradients(&particles);

                let mut damp = 0.0;
                let mut scale = 0.0;

                for (i, part) in particles.iter().enumerate() {
                    damp += gradients[i].dot(part.pos - part.prev_pos);
                    scale += part.inverse_mass * gradients[i].mag_squared();
                }

                let lagrange = (-evaluated - gamma * damp) / ((1.0 + gamma) * scale + alpha);

                for (i, part) in self.xpbd.particles().iter().enumerate() {
                    let displacement =
                        lagrange * part.get(particle_source).inverse_mass * gradients[i];
                    let pos = part.get(particle_source).pos;

                    part.get_mut(particle_source).add_displacement(
                        displacement,
                        pos,
                        self.as_force && !static_pass,
                        dt,
                    );
                }

                let force = lagrange / dt.powi(2);
                self.force = force;
                if breakable {
                    self.broken = force > max_force;
                }
            }
        }
    }
}
