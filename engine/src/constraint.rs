use crate::{
    math::Vec3,
    particle::{Force, Particle, ParticleReference},
};

//---------------------------------------------------------------------------------------------------//

pub enum ConstraintType {
    Equation,
    Inequality,
}

pub struct ConstraintProperties {
    particles: Vec<ParticleReference>,

    compliance: f64,
    dissipation: f64,

    force: f64,
    max_force: Option<f64>,
    broken: bool,

    xpbd: bool,
    constraint_type: ConstraintType,
    // projection_type: ProjectionType,
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

            force: 0.0,
            max_force: None,
            broken: false,

            xpbd,
            constraint_type,
        }
    }
}

//---------------------------------------------------------------------------------------------------//
// Constraint traits.

pub trait Constraint {
    fn project(&mut self, particle_source: &mut [Particle], dt: f64, static_pass: bool);
    fn force_estimate(&self) -> f64;
}

pub trait XPBD {
    fn properties(&self) -> &ConstraintProperties;
    fn properties_mut(&mut self) -> &mut ConstraintProperties;
    fn constraint(&self, particles: &[&Particle]) -> f64;
    fn gradients(&self, particles: &[&Particle]) -> Vec<Vec3>;
}

impl<C: XPBD> Constraint for C {
    fn project(&mut self, particle_source: &mut [Particle], dt: f64, static_pass: bool) {
        let data = self.properties();
        let (breakable, max_force) = match data.max_force {
            Some(force) => (true, force),
            None => (false, 0.0),
        };

        if !breakable || !data.broken {
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

                for (i, part) in particles.iter().enumerate() {
                    damp += gradients[i].dot(part.pos - part.prev_pos);
                    scale += part.inverse_mass * gradients[i].mag_squared();
                }

                let lagrange = (-evaluated - gamma * damp) / ((1.0 + gamma) * scale + alpha);

                for (i, part) in data.particles.iter().enumerate() {
                    if data.xpbd || static_pass {
                        let inv_mass = part.get(particle_source).inverse_mass;
                        part.get_mut(particle_source).pos += lagrange * inv_mass * gradients[i];
                    } else {
                        part.get_mut(particle_source)
                            .forces
                            .push(Force(lagrange * gradients[i] / dt.powi(2), None))
                    }
                }

                let force = lagrange / dt.powi(2);
                self.properties_mut().force = force;
                if breakable {
                    self.properties_mut().broken = force > max_force;
                }
            }
        }
    }

    fn force_estimate(&self) -> f64 {
        self.properties().force
    }
}

//---------------------------------------------------------------------------------------------------//
// Different constraints implemented using the Constraint traits.

pub mod builtin_constraints {
    use crate::{
        constraint::{ConstraintProperties, ConstraintType, XPBD},
        math::{Point3, Vec3},
        particle::{Particle, ParticleReference},
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

        pub fn max_tension(mut self, force: f64) -> Distance {
            self.data.max_force = Some(force);
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

    impl XPBD for Distance {
        fn properties(&self) -> &ConstraintProperties {
            &self.data
        }

        fn properties_mut(&mut self) -> &mut ConstraintProperties {
            &mut self.data
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

    pub struct NonPenetrate(ConstraintProperties, f64);

    impl NonPenetrate {
        pub fn new(
            particles: [ParticleReference; 2],
            collision_distance: f64,
            xpbd: bool,
        ) -> NonPenetrate {
            NonPenetrate(
                ConstraintProperties::new(particles.to_vec(), ConstraintType::Inequality, xpbd),
                collision_distance,
            )
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

    impl XPBD for NonPenetrate {
        fn properties(&self) -> &ConstraintProperties {
            &self.0
        }

        fn properties_mut(&mut self) -> &mut ConstraintProperties {
            &mut self.0
        }

        fn constraint(&self, particles: &[&Particle]) -> f64 {
            (particles[1].pos - particles[0].pos).mag() - self.1
        }

        fn gradients(&self, particles: &[&Particle]) -> Vec<Vec3> {
            let norm = (particles[1].pos - particles[0].pos).norm();
            vec![-norm, norm]
        }
    }

    //--------------------------------------------------------------------//

    pub struct ContactPlane {
        data: ConstraintProperties,
        collision_distance: f64,
        point: Point3,
        normal: Vec3,
    }

    impl ContactPlane {
        pub fn new(
            particle: ParticleReference,
            collision_distance: f64,
            point: Point3,
            normal: Vec3,
            xpbd: bool,
        ) -> ContactPlane {
            ContactPlane {
                data: ConstraintProperties::new(vec![particle], ConstraintType::Inequality, xpbd),
                collision_distance,
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

    impl XPBD for ContactPlane {
        fn properties(&self) -> &ConstraintProperties {
            &self.data
        }

        fn properties_mut(&mut self) -> &mut ConstraintProperties {
            &mut self.data
        }

        fn constraint(&self, particles: &[&Particle]) -> f64 {
            (particles[0].pos - self.point).dot(self.normal) - self.collision_distance
        }

        fn gradients(&self, _particles: &[&Particle]) -> Vec<Vec3> {
            vec![self.normal]
        }
    }

    //--------------------------------------------------------------------//
}
