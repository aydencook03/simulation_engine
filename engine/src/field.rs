//! An object that enables dynamic interaction with and between particles.
//! A field can also store its own state and have an integration method.

use crate::{
    math::Vec3,
    particle::{Particle, ParticleReference},
};

//---------------------------------------------------------------------------------------------------//
// FieldProperties

pub enum InteractionType {
    FieldParticle,
    ParticleParticle,
    SimpleForce,
}

pub struct FieldProperties {
    coupled_particles: Vec<ParticleReference>,
    interaction_type: InteractionType,
}

impl FieldProperties {
    pub fn new(interaction_type: InteractionType) -> FieldProperties {
        FieldProperties {
            coupled_particles: Vec::new(),
            interaction_type,
        }
    }
}

//---------------------------------------------------------------------------------------------------//
// Field traits.

pub trait Field {
    fn handle(&mut self, particle_source: &mut [Particle], dt: f64);
    fn total_energy(&self, particle_source: &[Particle]) -> f64;

    fn add_particle(&mut self, _particle_reference: ParticleReference);
    fn add_particles(&mut self, _particle_references: &[ParticleReference]);
}

pub trait FieldImpl {
    fn properties(&self) -> &FieldProperties;
    fn properties_mut(&mut self) -> &mut FieldProperties;

    // for FieldParticle interaction type
    fn particle_to_field(&mut self, _particle: &Particle) {}
    fn integrate(&mut self, _dt: f64) {}
    fn field_to_particle(&self, _particle: &Particle) -> Option<Vec3> {
        None
    }
    fn clear(&mut self) {}
    fn field_energy(&self) -> f64 {
        0.0
    }

    // for ParticleParticle interaction type
    fn particle_to_particle(&self, _particle1: &Particle, _particle2: &Particle) -> Option<Vec3> {
        None
    }
    fn interaction_potential(&self, _particle1: &Particle, _particle2: &Particle) -> f64 {
        0.0
    }

    // for SimpleForce interaction type
    fn simple_force(&self, _particle: &Particle) -> Option<Vec3> {
        None
    }
    fn force_potential(&self, _particle: &Particle) -> f64 {
        0.0
    }
}

//--------------------------------------------------------------------//

impl<F: FieldImpl> Field for F {
    fn handle(&mut self, particle_source: &mut [Particle], dt: f64) {
        match &self.properties().interaction_type {
            InteractionType::FieldParticle => {
                // ready field for fresh update
                self.clear();

                // particles -> act on field
                for reference in &self.properties().coupled_particles.to_owned() {
                    // need to find a way around the ".to_owned()"
                    self.particle_to_field(reference.get(particle_source));
                }

                // field dynamics
                self.integrate(dt);

                // field -> act on particles
                for reference in &self.properties().coupled_particles {
                    let particle = reference.get_mut(particle_source);
                    if let Some(force) = self.field_to_particle(particle) {
                        particle.forces.push(force);
                    }
                }
            }
            InteractionType::ParticleParticle => {
                let mut index: usize = 0;
                for ref1 in &self.properties().coupled_particles {
                    for ref2 in &self.properties().coupled_particles[(index + 1)..] {
                        if let Some(force) = self.particle_to_particle(
                            ref1.get(particle_source),
                            ref2.get(particle_source),
                        ) {
                            ref1.get_mut(particle_source).forces.push(force);
                            ref2.get_mut(particle_source).forces.push(-force);
                        }
                    }
                    index += 1;
                }
            }
            InteractionType::SimpleForce => {
                for reference in &self.properties().coupled_particles {
                    if let Some(force) = self.simple_force(reference.get(particle_source)) {
                        reference.get_mut(particle_source).forces.push(force);
                    }
                }
            }
        }
    }

    fn total_energy(&self, particle_source: &[Particle]) -> f64 {
        let properties = &self.properties();
        match self.properties().interaction_type {
            InteractionType::FieldParticle => self.field_energy(),
            InteractionType::ParticleParticle => {
                let mut potential = 0.0;
                let mut index: usize = 0;
                for ref1 in &properties.coupled_particles {
                    for ref2 in &properties.coupled_particles[(index + 1)..] {
                        potential += self.interaction_potential(
                            ref1.get(particle_source),
                            ref2.get(particle_source),
                        );
                    }
                    index += 1;
                }
                potential
            }
            InteractionType::SimpleForce => {
                let mut potential = 0.0;
                for reference in &properties.coupled_particles {
                    potential += self.force_potential(reference.get(particle_source));
                }
                potential
            }
        }
    }

    fn add_particle(&mut self, particle_reference: ParticleReference) {
        self.properties_mut()
            .coupled_particles
            .push(particle_reference);
    }

    fn add_particles(&mut self, particle_references: &[ParticleReference]) {
        for reference in particle_references {
            self.properties_mut().coupled_particles.push(*reference);
        }
    }
}

//---------------------------------------------------------------------------------------------------//
// Different fields implemented using the field traits.

pub mod builtin_fields {
    use crate::{
        field::{FieldImpl, FieldProperties, InteractionType},
        math::Vec3,
        particle::Particle,
    };

    //--------------------------------------------------------------------//

    pub struct ConstantForce(FieldProperties, Vec3);

    impl ConstantForce {
        pub fn new(force: Vec3) -> ConstantForce {
            ConstantForce(FieldProperties::new(InteractionType::SimpleForce), force)
        }
    }

    impl FieldImpl for ConstantForce {
        fn properties(&self) -> &FieldProperties {
            &self.0
        }
        fn properties_mut(&mut self) -> &mut FieldProperties {
            &mut self.0
        }
        fn simple_force(&self, _particle: &Particle) -> Option<Vec3> {
            Some(self.1)
        }
    }

    //--------------------------------------------------------------------//

    pub struct Falling(FieldProperties, f64, f64);

    impl Falling {
        pub fn new(g: f64) -> Falling {
            Falling(FieldProperties::new(InteractionType::SimpleForce), g, 0.0)
        }

        pub fn ground_reference(mut self, height: f64) -> Falling {
            self.2 = height;
            self
        }
    }

    impl FieldImpl for Falling {
        fn properties(&self) -> &FieldProperties {
            &self.0
        }
        fn properties_mut(&mut self) -> &mut FieldProperties {
            &mut self.0
        }
        fn simple_force(&self, particle: &Particle) -> Option<Vec3> {
            Some(Vec3::new(0.0, -particle.mass * self.1, 0.0))
        }
        fn force_potential(&self, particle: &Particle) -> f64 {
            particle.mass * self.1 * (particle.pos.y - self.2)
        }
    }

    //--------------------------------------------------------------------//

    pub struct Gravity(FieldProperties, f64);

    impl Gravity {
        pub fn new(gravitational_constant: f64) -> Gravity {
            Gravity(
                FieldProperties::new(InteractionType::ParticleParticle),
                gravitational_constant,
            )
        }
    }

    impl FieldImpl for Gravity {
        fn properties(&self) -> &FieldProperties {
            &self.0
        }
        fn properties_mut(&mut self) -> &mut FieldProperties {
            &mut self.0
        }
        fn particle_to_particle(&self, particle1: &Particle, particle2: &Particle) -> Option<Vec3> {
            let radial = particle2.pos - particle1.pos;
            let dist = radial.mag();

            Some(((self.1 * particle1.mass * particle2.mass) / dist.powi(3)) * radial)
        }
        fn interaction_potential(&self, particle1: &Particle, particle2: &Particle) -> f64 {
            let radial = particle2.pos - particle1.pos;
            -self.1 * particle1.mass * particle2.mass / radial.mag()
        }
    }

    //--------------------------------------------------------------------//

    pub struct ElectroStatic(FieldProperties, f64);

    impl ElectroStatic {
        pub fn new(electrostatic_constant: f64) -> ElectroStatic {
            ElectroStatic(
                FieldProperties::new(InteractionType::ParticleParticle),
                electrostatic_constant,
            )
        }
    }

    impl FieldImpl for ElectroStatic {
        fn properties(&self) -> &FieldProperties {
            &self.0
        }

        fn properties_mut(&mut self) -> &mut FieldProperties {
            &mut self.0
        }
        fn particle_to_particle(&self, particle1: &Particle, particle2: &Particle) -> Option<Vec3> {
            let radial = particle2.pos - particle1.pos;
            let dist = radial.mag();

            Some(-((self.1 * particle1.charge * particle2.charge) / dist.powi(3)) * radial)
        }

        fn interaction_potential(&self, particle1: &Particle, particle2: &Particle) -> f64 {
            let radial = particle2.pos - particle1.pos;
            self.1 * particle1.charge * particle2.charge / radial.mag()
        }
    }

    //--------------------------------------------------------------------//

    pub struct LennardJones {
        properties: FieldProperties,
        dispersion_energy: f64,
        collision_radius: f64,
        repulsion: f64,
        attraction: f64,
    }

    impl LennardJones {
        pub fn new(dispersion_energy: f64, collision_radius: f64) -> LennardJones {
            LennardJones {
                properties: FieldProperties::new(InteractionType::ParticleParticle),
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
    }

    impl FieldImpl for LennardJones {
        fn properties(&self) -> &FieldProperties {
            &self.properties
        }
        fn properties_mut(&mut self) -> &mut FieldProperties {
            &mut self.properties
        }
        fn particle_to_particle(&self, particle1: &Particle, particle2: &Particle) -> Option<Vec3> {
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
        fn interaction_potential(&self, particle1: &Particle, particle2: &Particle) -> f64 {
            let (n, m) = (self.repulsion, self.attraction);
            let c = (n / (n - m)) * ((n / m).powf(m / (n - m))) * self.dispersion_energy;
            let sigma = self.collision_radius;
            let r = (particle2.pos - particle1.pos).mag();

            c * ((sigma / r).powf(n) - (sigma / r).powf(m))
        }
    }
}

//---------------------------------------------------------------------------------------------------//
