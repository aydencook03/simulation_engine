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
        fn project(&mut self, particle_source: &mut [Particle], dt: f64, _is_static: bool) {
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
                        - norm.dot(particle2.pos - particle2.prev_pos)))
                / ((1.0 + gamma) * (inv_mass1 + inv_mass2) + alpha);
            self.particles[0].get_mut(particle_source).pos += lagrange * inv_mass1 * norm;
            //println!("Force: {:#?}", (-lagrange*inv_mass2/dt.powi(2))*norm);
            self.particles[1].get_mut(particle_source).pos += -lagrange * inv_mass2 * norm;
        }
    }

    //--------------------------------------------------------------------//

    pub struct NonPenetrate {
        particles: [ParticleReference; 2],
        immediate: bool,
    }

    impl NonPenetrate {
        pub fn new(particles: [ParticleReference; 2], immediate: bool) -> NonPenetrate {
            NonPenetrate {
                particles,
                immediate,
            }
        }
    }

    impl Constraint for NonPenetrate {
        fn project(&mut self, particle_source: &mut [Particle], _dt: f64, is_static: bool) {
            let particle1 = self.particles[0].get(particle_source);
            let particle2 = self.particles[1].get(particle_source);
            let radial = particle2.pos - particle1.pos;
            let correction = (particle1.radius + particle2.radius) - radial.mag();

            if correction > 0.0 {
                let norm = radial.norm();
                let inv_mass1 = particle1.inverse_mass();
                let inv_mass2 = particle2.inverse_mass();
                let lagrange = (-correction) / (inv_mass1 + inv_mass2);
                if self.immediate || is_static {
                    self.particles[0].get_mut(particle_source).pos += lagrange * inv_mass1 * norm;
                    self.particles[1].get_mut(particle_source).pos += -lagrange * inv_mass2 * norm;
                } else {
                    self.particles[0]
                        .get_mut(particle_source)
                        .displacements
                        .push(lagrange * inv_mass1 * norm);
                    self.particles[1]
                        .get_mut(particle_source)
                        .displacements
                        .push(lagrange * inv_mass2 * norm);
                }
            }
        }
    }

    //--------------------------------------------------------------------//

    pub struct ContactPlane {
        particle: ParticleReference,
        point: Vec3,
        normal: Vec3,
        immediate: bool,
    }

    impl ContactPlane {
        pub fn new(
            particle: ParticleReference,
            point: Vec3,
            normal: Vec3,
            immediate: bool,
        ) -> ContactPlane {
            ContactPlane {
                particle,
                point,
                normal,
                immediate,
            }
        }
    }

    impl Constraint for ContactPlane {
        fn project(&mut self, particle_source: &mut [Particle], _dt: f64, is_static: bool) {
            let particle = self.particle.get_mut(particle_source);
            let norm = self.normal.norm();
            let dist = (particle.pos - self.point).dot(norm) - particle.radius;

            if dist < 0.0 {
                if self.immediate || is_static {
                    particle.pos += -dist * norm * particle.inverse_mass() * particle.mass;
                } else {
                    particle
                        .displacements
                        .push(-dist * norm * particle.inverse_mass() * particle.mass);
                }
            }
        }
    }
}

//---------------------------------------------------------------------------------------------------//
/*
pub enum Constraint {
    Distance([ParticleReference; 2], f64, f64, f64),
    NonPenetrate([ParticleReference; 2]),
}

impl Constraint {
    pub fn project(&self, particle_source: &mut [Particle], dt: f64) {
        match &self {
            Constraint::Distance(refs, dist, comp, diss) => {
                let particle1 = refs[0].get(particle_source);
                let particle2 = refs[1].get(particle_source);
                let inv_mass1 = particle1.inverse_mass();
                let inv_mass2 = particle2.inverse_mass();
                let alpha = comp / dt.powi(2);
                let gamma = comp * diss / dt;
                let radial = particle2.pos - particle1.pos;
                let norm = radial.norm();
                let correction = dist - radial.mag();
                let lagrange = (-correction
                    - gamma
                        * (norm.dot(particle1.pos - particle1.prev_pos)
                            - norm.dot(particle2.pos - particle2.prev_pos)))
                    / ((1.0 + gamma) * (inv_mass1 + inv_mass2) + alpha);
                refs[0].get_mut(particle_source).pos += lagrange * inv_mass1 * norm;
                //println!("Force: {:#?}", (-lagrange*inv_mass2/dt.powi(2))*norm);
                refs[1].get_mut(particle_source).pos += -lagrange * inv_mass2 * norm;
            }
            Constraint::NonPenetrate(refs) => {
                let particle1 = refs[0].get(particle_source);
                let particle2 = refs[1].get(particle_source);
                let radial = particle2.pos - particle1.pos;
                let correction = (particle1.radius + particle2.radius) - radial.mag();

                if correction > 0.0 {
                    let norm = radial.norm();
                    let inv_mass1 = particle1.inverse_mass();
                    let inv_mass2 = particle2.inverse_mass();
                    let lagrange = (-correction) / (inv_mass1 + inv_mass2);
                    refs[0].get_mut(particle_source).pos += lagrange * inv_mass1 * norm;
                    refs[1].get_mut(particle_source).pos += -lagrange * inv_mass2 * norm;
                }
            }
        }
    }
}
*/
//---------------------------------------------------------------------------------------------------//
/*
pub struct ConstraintData<const PARTICLE_COUNT: usize> {
    pub constrained_particles: [ParticleReference; PARTICLE_COUNT],
    pub compliance: f64,
    pub dissipation: f64,

    pub constraint_function: Box<dyn Fn([&Particle; PARTICLE_COUNT]) -> f64>,
    pub constraint_gradient: Box<dyn Fn(&Particle) -> Vec3>,
}

impl<const PARTICLE_COUNT: usize> ConstraintData<PARTICLE_COUNT> {
    fn project(&self, particle_source: &mut [Particle], dt: f64) {
        let mut particles: [&Particle; PARTICLE_COUNT] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for (index, reference) in self.constrained_particles.iter().enumerate() {
            particles[index] = reference.get(particle_source);
        }

        let alpha = self.compliance / dt.powi(2);
        let gamma = self.compliance * self.dissipation / dt;
        let mut damp = 0.0;
        let mut grad = 0.0;

        for reference in &self.constrained_particles {
            let particle = reference.get(particle_source);
            let gradient = (*self.constraint_gradient)(particle);
            damp += gradient.dot(particle.pos - particle.prev_pos);
            grad += particle.inverse_mass() * gradient.mag_squared();
        }

        let lagrange = (-(*self.constraint_function)(particles) - gamma * damp)
            / ((gamma + 1.0) * grad + alpha);

        for reference in &self.constrained_particles {
            let particle = reference.get_mut(particle_source);
            particle.pos +=
                lagrange * particle.inverse_mass() * (*self.constraint_gradient)(particle);
        }
    }
}

//---------------------------------------------------------------------------------------------------//

pub trait Constraint<const PARTICLE_COUNT: usize> {
    fn data(&mut self) -> &mut ConstraintData<PARTICLE_COUNT>;
}

//---------------------------------------------------------------------------------------------------//

impl<const PARTICLE_COUNT: usize> dyn Constraint<PARTICLE_COUNT> {
    pub fn handle(&mut self, particle_source: &mut [Particle], dt: f64) {
        self.data().project(particle_source, dt);
    }
}
*/
//---------------------------------------------------------------------------------------------------//
