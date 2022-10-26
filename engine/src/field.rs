//! An object that enables dynamic interaction with and between particles.
//! A field can also store its own state and have an integration method.

use crate::particle::{Particle, ParticleReference};
use crate::vec3::Vec3;

//---------------------------------------------------------------------------------------------------//
// CoupledParticles.

pub struct CoupledParticles(pub Vec<ParticleReference>);

impl CoupledParticles {
    pub fn new() -> CoupledParticles {
        CoupledParticles(Vec::new())
    }
}

//---------------------------------------------------------------------------------------------------//
// ParticleAction.

#[derive(Default, Copy, Clone)]
pub struct ParticleAction {
    force: Option<Vec3>,
    impulse: Option<Vec3>,
    displacement: Option<Vec3>,
    internal_work: Option<f64>,
}

impl ParticleAction {
    pub fn new() -> ParticleAction {
        ParticleAction::default()
    }
    pub fn force(mut self, force: Vec3) -> ParticleAction {
        self.force = Some(force);
        self
    }
    pub fn impulse(mut self, impulse: Vec3) -> ParticleAction {
        self.impulse = Some(impulse);
        self
    }
    pub fn displacement(mut self, displacement: Vec3) -> ParticleAction {
        self.displacement = Some(displacement);
        self
    }
    pub fn work(mut self, work: f64) -> ParticleAction {
        self.internal_work = Some(work);
        self
    }

    pub fn flipped(mut self) -> ParticleAction {
        if let Some(force) = self.force {
            self.force = Some(-1. * force);
        }
        if let Some(impulse) = self.impulse {
            self.impulse = Some(-1. * impulse)
        }
        if let Some(displacement) = self.displacement {
            self.displacement = Some(-1. * displacement);
        }
        self
    }

    pub fn send_to_particle(&self, particle: &mut Particle) {
        if let Some(force) = self.force {
            particle.forces.push(force);
        }
        if let Some(impulse) = self.impulse {
            particle.impulses.push(impulse);
        }
        if let Some(displacement) = self.displacement {
            particle.displacements.push(displacement);
        }
    }
}

//---------------------------------------------------------------------------------------------------//
// InteractionType.

/// # Describes the way the field interacts with its coupled particles.
///
/// ### Field ⇄ Particle:
/// -
///
/// ### Particle ⇄ Particle:
/// - Field doesn't store any state of its own.
/// - The force on each particle only depends on the current state of the other particles.
#[derive(Copy, Clone)]
pub enum InteractionType {
    FieldParticle,
    ParticleParticle,
}

//---------------------------------------------------------------------------------------------------//
// Field trait.

pub trait Field {
    fn coupled_particles(&self) -> &CoupledParticles;
    fn coupled_particles_mut(&mut self) -> &mut CoupledParticles;
    fn interaction_type(&self) -> InteractionType;

    fn add_particle(&mut self, particle_reference: ParticleReference) {
        self.coupled_particles_mut().0.push(particle_reference);
    }
    fn add_particles(&mut self, particle_references: &[ParticleReference]) {
        for reference in particle_references {
            self.coupled_particles_mut().0.push(*reference);
        }
    }

    fn particle_to_field(&mut self, _particle: &Particle) {}
    fn integrate(&mut self, _dt: f64) {}
    fn field_to_particle(&self, _particle: &Particle) -> ParticleAction {
        ParticleAction::new()
    }
    fn clear(&mut self) {}

    fn particle_to_particle(&self, _particle1: &Particle, _particle2: &Particle) -> ParticleAction {
        ParticleAction::new()
    }
}

//--------------------------------------------------------------------//

impl dyn Field {
    pub fn handle(&mut self, particles: &mut [Particle], dt: f64) {
        match self.interaction_type() {
            InteractionType::FieldParticle => {
                // ready field for fresh update
                self.clear();

                // particles -> act on field
                for reference in &self.coupled_particles().0.to_owned() {
                    // need to find a way around the ".to_owned()"
                    self.particle_to_field(reference.get(particles));
                }

                // field dynamics
                self.integrate(dt);

                // field -> act on particles
                for reference in &self.coupled_particles().0 {
                    let particle = reference.get_mut(particles);
                    let action = self.field_to_particle(particle);
                    action.send_to_particle(particle);
                }
            }
            InteractionType::ParticleParticle => {
                let mut index: usize = 0;
                for ref1 in &self.coupled_particles().0 {
                    for ref2 in &self.coupled_particles().0[(index + 1)..] {
                        let action =
                            self.particle_to_particle(ref1.get(particles), ref2.get(particles));
                        action.send_to_particle(ref1.get_mut(particles));
                        action.flipped().send_to_particle(ref2.get_mut(particles));
                    }
                    index += 1;
                }
            }
        }
    }
}

//---------------------------------------------------------------------------------------------------//
// Different fields implemented using the Field trait.

pub mod builtin_fields {
    use crate::{
        field::{CoupledParticles, Field, InteractionType, ParticleAction},
        particle::{Particle, Vec3},
    };

    //--------------------------------------------------------------------//

    pub struct ConstantForce(CoupledParticles, Vec3);

    impl ConstantForce {
        pub fn new(force: Vec3) -> ConstantForce {
            ConstantForce(CoupledParticles::new(), force)
        }
    }

    impl Field for ConstantForce {
        fn coupled_particles(&self) -> &CoupledParticles {
            &self.0
        }
        fn coupled_particles_mut(&mut self) -> &mut CoupledParticles {
            &mut self.0
        }
        fn interaction_type(&self) -> InteractionType {
            InteractionType::FieldParticle
        }
        fn field_to_particle(&self, _particle: &Particle) -> ParticleAction {
            ParticleAction::new().force(self.1)
        }
    }

    //--------------------------------------------------------------------//

    pub struct Falling(CoupledParticles, f64);

    impl Falling {
        pub fn new(g: f64) -> Falling {
            Falling(CoupledParticles::new(), g)
        }
    }

    impl Field for Falling {
        fn coupled_particles(&self) -> &CoupledParticles {
            &self.0
        }
        fn coupled_particles_mut(&mut self) -> &mut CoupledParticles {
            &mut self.0
        }
        fn interaction_type(&self) -> InteractionType {
            InteractionType::FieldParticle
        }
        fn field_to_particle(&self, particle: &Particle) -> ParticleAction {
            ParticleAction::new().force(Vec3::new(0.0, -particle.mass * self.1, 0.0))
        }
    }

    //--------------------------------------------------------------------//

    pub struct Gravity(CoupledParticles, f64, f64);

    impl Gravity {
        pub fn new(gravitational_constant: f64, softening_parameter: f64) -> Gravity {
            Gravity(
                CoupledParticles::new(),
                gravitational_constant,
                softening_parameter,
            )
        }
    }

    impl Field for Gravity {
        fn coupled_particles(&self) -> &CoupledParticles {
            &self.0
        }
        fn coupled_particles_mut(&mut self) -> &mut CoupledParticles {
            &mut self.0
        }
        fn interaction_type(&self) -> InteractionType {
            InteractionType::ParticleParticle
        }
        fn particle_to_particle(
            &self,
            particle1: &Particle,
            particle2: &Particle,
        ) -> ParticleAction {
            let radial = particle1.pos - particle2.pos;
            let dist_sqr = radial.mag_squared();

            ParticleAction::new().force(
                -(self.1 * particle1.mass * particle2.mass)
                    / (dist_sqr + self.2.powi(2)).powi(3).sqrt()
                    * radial,
            )
        }
    }

    //--------------------------------------------------------------------//

    pub struct VanDerWaals(CoupledParticles, f64, Option<f64>, f64);

    impl VanDerWaals {
        pub fn new(
            bond_energy: f64,
            bond_length: Option<f64>,
            softening_parameter: f64,
        ) -> VanDerWaals {
            VanDerWaals(
                CoupledParticles::new(),
                bond_energy,
                bond_length,
                softening_parameter,
            )
        }
    }

    impl Field for VanDerWaals {
        fn coupled_particles(&self) -> &CoupledParticles {
            &self.0
        }
        fn coupled_particles_mut(&mut self) -> &mut CoupledParticles {
            &mut self.0
        }
        fn interaction_type(&self) -> InteractionType {
            InteractionType::ParticleParticle
        }
        fn particle_to_particle(
            &self,
            particle1: &Particle,
            particle2: &Particle,
        ) -> ParticleAction {
            let radial = particle1.pos - particle2.pos;
            let dist_sqr = radial.mag_squared();
            let bond_6 = if let Some(length) = self.2 {
                length.powi(6)
            } else {
                (particle1.radius + particle2.radius).powi(6)
            };
            let bond_12 = bond_6.powi(2);

            let denom = dist_sqr + self.3.powi(2);
            ParticleAction::new().force(
                (-12_f64 * self.1 * (bond_6 / denom.powi(4) - bond_12 / denom.powi(7))) * radial,
            )
        }
    }
}

//---------------------------------------------------------------------------------------------------//
