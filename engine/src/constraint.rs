use crate::{
    math::Vec3,
    particle::{Particle, ParticleReference},
};

//---------------------------------------------------------------------------------------------------//

pub struct Constraint {
    constraint_type: ConstraintType,
}

pub enum ConstraintType {
    Generic(Box<dyn GenericConstraint>),
    Xpbd(XpbdConstraint),
    Boundary(Box<dyn BoundaryConstraint>),
}

impl Constraint {
    pub fn new(constraint_type: ConstraintType) -> Constraint {
        Constraint { constraint_type }
    }

    //--------------------------------------------------------------------//

    pub fn project(&mut self, particle_source: &mut [Particle], dt: f64, static_pass: bool) {
        match &mut self.constraint_type {
            ConstraintType::Generic(constraint) => {
                constraint.project(particle_source, dt, static_pass)
            }
            ConstraintType::Xpbd(constraint) => {
                let (breakable, max_force) = match constraint.max_force {
                    Some(force) => (true, force),
                    None => (false, 0.0),
                };

                if !breakable || !constraint.broken {
                    let particles: Vec<&Particle> = constraint
                        .implementation
                        .particles()
                        .iter()
                        .map(|p| p.get(particle_source))
                        .collect();

                    let evaluated = constraint.implementation.constraint(&particles);

                    let satisfied = match constraint.as_inequality {
                        false => evaluated == 0.0,
                        true => evaluated >= 0.0,
                    };

                    if !satisfied {
                        let dt = if static_pass { core::f64::MAX } else { dt };
                        let alpha = constraint.compliance / dt.powi(2);
                        let gamma = constraint.compliance * constraint.dissipation / dt;
                        let gradients = constraint.implementation.gradients(&particles);

                        let mut damp = 0.0;
                        let mut scale = 0.0;

                        for (i, part) in particles.iter().enumerate() {
                            damp += gradients[i].dot(part.pos - part.prev_pos);
                            scale += part.inverse_mass * gradients[i].mag_squared();
                        }

                        let lagrange =
                            (-evaluated - gamma * damp) / ((1.0 + gamma) * scale + alpha);

                        for (i, part) in constraint.implementation.particles().iter().enumerate() {
                            if !constraint.as_force || static_pass {
                                let inv_mass = part.get(particle_source).inverse_mass;
                                part.get_mut(particle_source).pos +=
                                    lagrange * inv_mass * gradients[i];
                            } else {
                                part.get_mut(particle_source)
                                    .forces
                                    .push(lagrange * gradients[i] / dt.powi(2));
                            }
                        }

                        let force = lagrange / dt.powi(2);
                        constraint.force = force;
                        if breakable {
                            constraint.broken = force > max_force;
                        }
                    }
                }
            }
            ConstraintType::Boundary(_constraint) => todo!(),
        }
    }

    pub fn force_estimate(&self) -> f64 {
        match &self.constraint_type {
            ConstraintType::Generic(_constraint) => 0.0,
            ConstraintType::Xpbd(constraint) => constraint.force,
            ConstraintType::Boundary(_constraint) => todo!(),
        }
    }
}

//---------------------------------------------------------------------------------------------------//

pub trait GenericConstraint {
    fn project(&mut self, particle_source: &mut [Particle], dt: f64, static_pass: bool);
}

//--------------------------------------------------------------------//

pub struct XpbdConstraint {
    implementation: Box<dyn XpbdImplementation>,

    compliance: f64,
    dissipation: f64,

    force: f64,
    max_force: Option<f64>,
    broken: bool,

    as_force: bool,
    as_inequality: bool,
}

impl XpbdConstraint {
    pub fn new(implementation: impl XpbdImplementation + 'static) -> XpbdConstraint {
        XpbdConstraint {
            implementation: Box::new(implementation),
            compliance: 0.0,
            dissipation: 0.0,
            force: 0.0,
            max_force: None,
            broken: false,
            as_force: false,
            as_inequality: false,
        }
    }

    pub fn compliance(mut self, compliance: f64) -> XpbdConstraint {
        self.compliance = compliance;
        self
    }

    pub fn dissipation(mut self, dissipation: f64) -> XpbdConstraint {
        self.dissipation = dissipation;
        self
    }

    pub fn max_force(mut self, max_force: f64) -> XpbdConstraint {
        self.max_force = Some(max_force);
        self
    }

    pub fn as_force(mut self) -> XpbdConstraint {
        self.as_force = true;
        self
    }

    pub fn as_inequality(mut self) -> XpbdConstraint {
        self.as_inequality = true;
        self
    }

    pub fn build(self) -> Constraint {
        Constraint::new(ConstraintType::Xpbd(self))
    }
}

pub trait XpbdImplementation {
    fn particles(&self) -> &[ParticleReference];
    fn constraint(&self, particles: &[&Particle]) -> f64;
    fn gradients(&self, particles: &[&Particle]) -> Vec<Vec3>;
}

/* future optimized version?
pub trait XPBDConstraint {
    const COUNT: usize;
    fn particles(&self) -> [ParticleReference; Self::COUNT];
    fn constraint(&self, particles: &[&Particle; Self::COUNT]) -> f64;
    fn gradients(&self, particles: &[&Particle; Self::COUNT]) -> [Vec3; Self::COUNT];
} */

//--------------------------------------------------------------------//

pub trait BoundaryConstraint {}

//---------------------------------------------------------------------------------------------------//

pub mod builtin_constraints {
    use crate::{
        constraint::{XpbdConstraint, XpbdImplementation},
        math::{Point3, Vec3},
        particle::{Particle, ParticleReference},
    };

    //--------------------------------------------------------------------//

    pub struct Distance([ParticleReference; 2], f64);

    impl Distance {
        pub fn new(particles: [ParticleReference; 2], dist: f64) -> XpbdConstraint {
            XpbdConstraint::new(Distance(particles, dist))
        }
    }

    impl XpbdImplementation for Distance {
        fn particles(&self) -> &[ParticleReference] {
            &self.0
        }

        fn constraint(&self, particles: &[&Particle]) -> f64 {
            self.1 - (particles[1].pos - particles[0].pos).mag()
        }

        fn gradients(&self, particles: &[&Particle]) -> Vec<Vec3> {
            let norm = (particles[1].pos - particles[0].pos).norm();
            vec![norm, -norm]
        }
    }

    //--------------------------------------------------------------------//

    pub struct NonPenetrate([ParticleReference; 2], f64);

    impl NonPenetrate {
        pub fn new(particles: [ParticleReference; 2], collision_distance: f64) -> XpbdConstraint {
            XpbdConstraint::new(NonPenetrate(particles, collision_distance)).as_inequality()
        }
    }

    impl XpbdImplementation for NonPenetrate {
        fn particles(&self) -> &[ParticleReference] {
            &self.0
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
        particle: [ParticleReference; 1],
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
        ) -> XpbdConstraint {
            XpbdConstraint::new(ContactPlane {
                particle: [particle],
                collision_distance,
                point,
                normal: normal.norm(),
            })
            .as_inequality()
        }
    }

    impl XpbdImplementation for ContactPlane {
        fn particles(&self) -> &[ParticleReference] {
            &self.particle
        }

        fn constraint(&self, particles: &[&Particle]) -> f64 {
            (particles[0].pos - self.point).dot(self.normal) - self.collision_distance
        }

        fn gradients(&self, _particles: &[&Particle]) -> Vec<Vec3> {
            vec![self.normal]
        }
    }
}

//---------------------------------------------------------------------------------------------------//
