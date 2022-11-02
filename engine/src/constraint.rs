use crate::{
    particle::{Particle, ParticleReference},
    vec3::Vec3,
};

// Old:                 New:
// star: 20-24 fps      star: 14-18 fps
// pile: 30-36 fps      pile: 14-16 fps

//---------------------------------------------------------------------------------------------------//

pub enum ConstraintType {
    Equation,
    Inequality,
}

pub struct ConstraintProperties {
    particles: Vec<ParticleReference>,
    compliance: f64,
    dissipation: f64,
    xpbd: bool,
    constraint_type: ConstraintType,
}

impl ConstraintProperties {
    pub fn new(
        particles: Vec<ParticleReference>,
        constraint_type: ConstraintType,
        xpbd: bool,
    ) -> ConstraintProperties {
        ConstraintProperties {
            particles,
            compliance: 0.0,
            dissipation: 0.0,
            xpbd,
            constraint_type,
        }
    }
}

//---------------------------------------------------------------------------------------------------//
// Constraint traits.

pub trait Constraint {
    fn project(&self, particle_source: &mut [Particle], dt: f64, static_pass: bool);
    // fn force_estimate
}

pub trait ConstraintData {
    fn properties(&self) -> &ConstraintProperties;
    fn constraint(&self, particles: &[&Particle]) -> f64;
    fn gradients(&self, particles: &[&Particle]) -> Vec<Vec3>;
}

impl<C: ConstraintData> Constraint for C {
    fn project(&self, particle_source: &mut [Particle], dt: f64, static_pass: bool) {
        let data = self.properties();
        let particles: Vec<&Particle> = data
            .particles
            .iter()
            .map(|p| p.get(particle_source))
            .collect();

        let evaluated = self.constraint(&particles);

        let satisfied = match data.constraint_type {
            ConstraintType::Equation => evaluated == 0.0,
            ConstraintType::Inequality => evaluated >= 0.0,
        };

        if !satisfied {
            let dt = if static_pass { core::f64::MAX } else { dt };
            let alpha = data.compliance / dt.powi(2);
            let gamma = data.compliance * data.dissipation / dt;
            let gradients = self.gradients(&particles);

            let mut damp = 0.0;
            let mut scale = 0.0;

            for (i, grad) in gradients.iter().enumerate() {
                damp += grad.dot(particles[i].pos - particles[i].prev_pos);
                scale += particles[i].inverse_mass() * grad.mag_squared();
            }

            let lagrange = (-evaluated - gamma * damp) / ((1.0 + gamma) * scale + alpha);

            let mut corrections = Vec::new();

            for (i, particle) in particles.iter().enumerate() {
                corrections.push(lagrange * particle.inverse_mass() * gradients[i]);
            }

            drop(particles);

            for (i, part) in data.particles.iter().enumerate() {
                if data.xpbd || static_pass {
                    part.get_mut(particle_source).pos += corrections[i];
                } else {
                    part.get_mut(particle_source)
                        .displacements
                        .push(corrections[i]);
                }
            }
        }
    }
}

//---------------------------------------------------------------------------------------------------//
// Different constraints implemented using the Constraint traits.

pub mod builtin_constraints {
    use crate::{
        constraint::{ConstraintData, ConstraintProperties, ConstraintType},
        particle::{Particle, ParticleReference},
        vec3::Vec3,
    };

    //--------------------------------------------------------------------//

    pub struct Distance {
        data: ConstraintProperties,
        dist: f64,
    }

    impl Distance {
        pub fn new(particles: [ParticleReference; 2], dist: f64) -> Distance {
            Distance {
                data: ConstraintProperties::new(particles.to_vec(), ConstraintType::Equation, true),
                dist,
            }
        }

        pub fn compliance(mut self, compliance: f64) -> Distance {
            self.data.compliance = compliance;
            self
        }

        pub fn dissipation(mut self, dissipation: f64) -> Distance {
            self.data.dissipation = dissipation;
            self
        }

        pub fn as_chain(mut self) -> Distance {
            self.data.constraint_type = ConstraintType::Inequality;
            self
        }

        pub fn as_non_xpbd(mut self) -> Distance {
            self.data.xpbd = false;
            self
        }
    }

    impl ConstraintData for Distance {
        fn properties(&self) -> &ConstraintProperties {
            &self.data
        }

        fn constraint(&self, particles: &[&Particle]) -> f64 {
            self.dist - (particles[1].pos - particles[0].pos).mag()
        }

        fn gradients(&self, particles: &[&Particle]) -> Vec<Vec3> {
            let norm = (particles[1].pos - particles[0].pos).norm();
            vec![norm, -norm]
        }
    }

    //--------------------------------------------------------------------//

    pub struct NonPenetrate(ConstraintProperties);

    impl NonPenetrate {
        pub fn new(particles: [ParticleReference; 2], xpbd: bool) -> NonPenetrate {
            NonPenetrate(ConstraintProperties::new(
                particles.to_vec(),
                ConstraintType::Inequality,
                xpbd,
            ))
        }

        pub fn compliance(mut self, compliance: f64) -> NonPenetrate {
            self.0.compliance = compliance;
            self
        }

        pub fn dissipation(mut self, dissipation: f64) -> NonPenetrate {
            self.0.dissipation = dissipation;
            self
        }
    }

    impl ConstraintData for NonPenetrate {
        fn properties(&self) -> &ConstraintProperties {
            &self.0
        }

        fn constraint(&self, particles: &[&Particle]) -> f64 {
            (particles[1].pos - particles[0].pos).mag()
                - (particles[0].radius + particles[1].radius)
        }

        fn gradients(&self, particles: &[&Particle]) -> Vec<Vec3> {
            let norm = (particles[1].pos - particles[0].pos).norm();
            vec![-norm, norm]
        }
    }

    //--------------------------------------------------------------------//

    pub struct ContactPlane {
        data: ConstraintProperties,
        point: Vec3,
        normal: Vec3,
    }

    impl ContactPlane {
        pub fn new(
            particle: ParticleReference,
            point: Vec3,
            normal: Vec3,
            xpbd: bool,
        ) -> ContactPlane {
            ContactPlane {
                data: ConstraintProperties::new(vec![particle], ConstraintType::Inequality, xpbd),
                point,
                normal: normal.norm(),
            }
        }

        pub fn compliance(mut self, compliance: f64) -> ContactPlane {
            self.data.compliance = compliance;
            self
        }

        pub fn dissipation(mut self, dissipation: f64) -> ContactPlane {
            self.data.dissipation = dissipation;
            self
        }
    }

    impl ConstraintData for ContactPlane {
        fn properties(&self) -> &ConstraintProperties {
            &self.data
        }

        fn constraint(&self, particles: &[&Particle]) -> f64 {
            (particles[0].pos - self.point).dot(self.normal) - particles[0].radius
        }

        fn gradients(&self, _particles: &[&Particle]) -> Vec<Vec3> {
            vec![self.normal]
        }
    }

    //--------------------------------------------------------------------//
}
