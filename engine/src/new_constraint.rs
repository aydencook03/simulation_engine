use crate::{
    particle::{Particle, ParticleReference},
    vec3::Vec3,
};

//---------------------------------------------------------------------------------------------------//

#[derive(Copy, Clone)]
pub enum ConstraintType {
    Equation,
    Inequality,
}

#[derive(Copy, Clone)]
pub struct ConstraintData<const COUNT: usize> {
    particles: [ParticleReference; COUNT],
    compliance: f64,
    dissipation: f64,
    xpbd: bool,
    constraint_type: ConstraintType,
}

//---------------------------------------------------------------------------------------------------//
// Constraint trait.

pub trait Constraint<const COUNT: usize> {
    fn data(&self) -> ConstraintData<COUNT>;
    fn constraint(&self, particles: [&Particle; COUNT]) -> f64;
    fn gradients(&self, particles: [&Particle; COUNT]) -> [Vec3; COUNT];

    fn project(&mut self, particle_source: &mut [Particle], dt: f64, static_pass: bool) {
        let data = self.data();
        let mut particles: [&Particle; COUNT] =
            unsafe { std::mem::MaybeUninit::uninit().assume_init() };

        for (index, reference) in data.particles.iter().enumerate() {
            particles[index] = reference.get(particle_source);
        }

        let evaluated = self.constraint(particles);

        let satisfied = match data.constraint_type {
            ConstraintType::Equation => evaluated == 0.0,
            ConstraintType::Inequality => evaluated >= 0.0,
        };

        if !satisfied {
            let dt = if static_pass { core::f64::MAX } else { dt };
            let alpha = data.compliance / dt.powi(2);
            let gamma = data.compliance * data.dissipation / dt;
            let gradients = self.gradients(particles);

            let mut damp = 0.0;
            let mut scale = 0.0;

            for (i, grad) in gradients.iter().enumerate() {
                damp += grad.dot(particles[i].pos - particles[i].prev_pos);
                scale += particles[i].inverse_mass() * grad.mag_squared();
            }

            let lagrange = (-evaluated - gamma * damp) / ((1.0 + gamma) * scale + alpha);

            let mut corrections = [Vec3::zero(); COUNT];

            for (i, particle) in particles.iter().enumerate() {
                corrections[i] = lagrange * particle.inverse_mass() * gradients[i];
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
// Different constraints implemented using the Constraint trait.

pub mod builtin_constraints {
    use crate::{
        constraint::{Constraint, ConstraintData, ConstraintType},
        particle::{Particle, ParticleReference},
        vec3::Vec3,
    };

    //--------------------------------------------------------------------//

    pub struct Distance {
        data: ConstraintData<2>,
        dist: f64,
    }

    impl Distance {
        pub fn new(particles: [ParticleReference; 2], dist: f64) -> Distance {
            Distance {
                data: ConstraintData {
                    particles,
                    compliance: 0.0,
                    dissipation: 0.0,
                    xpbd: true,
                    constraint_type: ConstraintType::Equation,
                },
                dist,
            }
        }
    }

    impl Constraint<2> for Distance {
        fn data(&self) -> ConstraintData<2> {
            self.data
        }

        fn constraint(&self, particles: [&Particle; 2]) -> f64 {
            self.dist.powi(2) - (particles[1].pos - particles[0].pos).mag_squared()
        }

        fn gradients(&self, particles: [&Particle; 2]) -> [Vec3; 2] {
            let norm = (particles[1].pos - particles[0].pos).norm();
            [norm, -norm]
        }
    }

    //--------------------------------------------------------------------//
}
