pub use crate::particle::{Particle, ParticleReference};
pub use crate::vec3::Vec3;

//---------------------------------------------------------------------------------------------------//
// Constraint trait.

pub trait Constraint {
    fn project(&mut self, particle_source: &mut [Particle], dt: f64, static_pass: bool);
    // fn force_estimate;
}

//---------------------------------------------------------------------------------------------------//
// Different constraints implemented using the Constraint trait.

pub mod builtin_constraints {
    use crate::{
        constraint::Constraint,
        particle::{Particle, ParticleReference},
        vec3::Vec3,
    };

    //--------------------------------------------------------------------//

    pub struct Distance {
        particles: [ParticleReference; 2],
        distance: f64,
        compliance: f64,
        dissipation: f64,
    }

    impl Distance {
        pub fn new(particles: [ParticleReference; 2], distance: f64) -> Distance {
            Distance {
                particles,
                distance,
                compliance: 0.0,
                dissipation: 0.0,
            }
        }

        pub fn compliance(mut self, compliance: f64) -> Distance {
            self.compliance = compliance;
            self
        }

        pub fn dissipation(mut self, dissipation: f64) -> Distance {
            self.dissipation = dissipation;
            self
        }
    }

    impl Constraint for Distance {
        fn project(&mut self, particle_source: &mut [Particle], dt: f64, static_pass: bool) {
            let dt = if static_pass { core::f64::MAX } else { dt };
            let particle1 = self.particles[0].get(particle_source);
            let particle2 = self.particles[1].get(particle_source);
            let inv_mass1 = particle1.inverse_mass();
            let inv_mass2 = particle2.inverse_mass();
            let alpha = self.compliance / dt.powi(2);
            let gamma = self.compliance * self.dissipation / dt;
            let radial = particle2.pos - particle1.pos;
            let norm = radial.norm();
            let correction = self.distance - radial.mag();
            let lagrange = (-correction
                - gamma
                    * (norm.dot(particle1.pos - particle1.prev_pos)
                        + (-norm).dot(particle2.pos - particle2.prev_pos)))
                / ((1.0 + gamma) * (inv_mass1 + inv_mass2) + alpha);
            self.particles[0].get_mut(particle_source).pos += lagrange * inv_mass1 * norm;
            self.particles[1].get_mut(particle_source).pos += lagrange * inv_mass2 * -norm;
        }
    }

    //--------------------------------------------------------------------//

    pub struct NonPenetrate {
        particles: [ParticleReference; 2],
        xpbd: bool,
        compliance: f64,
        dissipation: f64,
    }

    impl NonPenetrate {
        pub fn new(particles: [ParticleReference; 2], xpbd: bool) -> NonPenetrate {
            NonPenetrate {
                particles,
                compliance: 0.0,
                dissipation: 0.0,
                xpbd,
            }
        }

        pub fn compliance(mut self, compliance: f64) -> NonPenetrate {
            self.compliance = compliance;
            self
        }

        pub fn dissipation(mut self, dissipation: f64) -> NonPenetrate {
            self.dissipation = dissipation;
            self
        }
    }

    impl Constraint for NonPenetrate {
        fn project(&mut self, particle_source: &mut [Particle], dt: f64, static_pass: bool) {
            let particle1 = self.particles[0].get(particle_source);
            let particle2 = self.particles[1].get(particle_source);
            let radial = particle2.pos - particle1.pos;
            let correction = (particle1.radius + particle2.radius) - radial.mag();

            if correction > 0.0 {
                let dt = if static_pass { core::f64::MAX } else { dt };
                let norm = radial.norm();
                let inv_mass1 = particle1.inverse_mass();
                let inv_mass2 = particle2.inverse_mass();
                let alpha = self.compliance / dt.powi(2);
                let gamma = self.compliance * self.dissipation / dt;
                let lagrange = (-correction
                    - gamma
                        * (norm.dot(particle1.pos - particle1.prev_pos)
                            + (-norm).dot(particle2.pos - particle2.prev_pos)))
                    / ((1.0 + gamma) * (inv_mass1 + inv_mass2) + alpha);

                let correction1 = lagrange * inv_mass1 * norm;
                let correction2 = lagrange * inv_mass2 * -norm;

                if self.xpbd || static_pass {
                    self.particles[0].get_mut(particle_source).pos += correction1;
                    self.particles[1].get_mut(particle_source).pos += correction2;
                } else {
                    self.particles[0]
                        .get_mut(particle_source)
                        .displacements
                        .push(correction1);
                    self.particles[1]
                        .get_mut(particle_source)
                        .displacements
                        .push(correction2);
                }
            }
        }
    }

    //--------------------------------------------------------------------//

    pub struct ContactPlane {
        particle: ParticleReference,
        point: Vec3,
        normal: Vec3,
        compliance: f64,
        dissipation: f64,
        xpbd: bool,
    }

    impl ContactPlane {
        pub fn new(
            particle: ParticleReference,
            point: Vec3,
            normal: Vec3,
            xpbd: bool,
        ) -> ContactPlane {
            ContactPlane {
                particle,
                point,
                normal: normal.norm(),
                compliance: 0.0,
                dissipation: 0.0,
                xpbd,
            }
        }

        pub fn compliance(mut self, compliance: f64) -> ContactPlane {
            self.compliance = compliance;
            self
        }

        pub fn dissipation(mut self, dissipation: f64) -> ContactPlane {
            self.dissipation = dissipation;
            self
        }
    }

    impl Constraint for ContactPlane {
        fn project(&mut self, particle_source: &mut [Particle], dt: f64, static_pass: bool) {
            let particle = self.particle.get_mut(particle_source);
            let dist = (particle.pos - self.point).dot(self.normal) - particle.radius;

            if dist < 0.0 {
                let dt = if static_pass { core::f64::MAX } else { dt };
                let inv_mass = particle.inverse_mass();
                let alpha = self.compliance / dt.powi(2);
                let gamma = self.compliance * self.dissipation / dt;
                let lagrange = (-dist - gamma * self.normal.dot(particle.pos - particle.prev_pos))
                    / ((1.0 + gamma) * inv_mass + alpha);

                let correction = lagrange * inv_mass * self.normal;

                if self.xpbd || static_pass {
                    particle.pos += correction;
                } else {
                    particle.displacements.push(correction);
                }
            }
        }
    }
}

//---------------------------------------------------------------------------------------------------//
