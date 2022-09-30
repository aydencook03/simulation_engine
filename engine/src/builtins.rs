use crate::{
    field::{Field, InteractionType, ParticleAction},
    particle::{Particle, ParticleReference, Vec3},
};

//---------------------------------------------------------------------------------------------------//

pub struct ConstantForce(Vec<ParticleReference>, Vec3);
impl ConstantForce {
    pub fn new(force: Vec3) -> ConstantForce {
        ConstantForce(Vec::new(), force)
    }
    pub fn add_particle(&mut self, particle_reference: ParticleReference) {
        self.0.push(particle_reference);
    }
}
impl Field for ConstantForce {
    fn coupled_particles(&self) -> &[ParticleReference] {
        &self.0
    }
    fn interaction_type(&self) -> InteractionType {
        InteractionType::FieldParticle
    }
    fn field_to_particle(&self, _particle: &Particle) -> ParticleAction {
        ParticleAction::new().force(self.1)
    }
}

//---------------------------------------------------------------------------------------------------//

pub struct Gravity(Vec<ParticleReference>, f64);
impl Gravity {
    pub fn new(g: f64) -> Gravity {
        Gravity(Vec::new(), g)
    }

    pub fn add_particle(&mut self, particle_reference: ParticleReference) {
        self.0.push(particle_reference);
    }
}
impl Field for Gravity {
    fn coupled_particles(&self) -> &[ParticleReference] {
        &self.0
    }
    fn interaction_type(&self) -> InteractionType {
        InteractionType::FieldParticle
    }
    fn field_to_particle(&self, particle: &Particle) -> ParticleAction {
        ParticleAction::new().force(Vec3::new(0.0, -particle.mass * self.1, 0.0))
    }
}

//---------------------------------------------------------------------------------------------------//

pub struct NGravity(Vec<ParticleReference>, f64);
impl NGravity {
    pub fn new(gravitational_constant: f64) -> NGravity {
        NGravity(Vec::new(), gravitational_constant)
    }

    pub fn add_particle(&mut self, particle_reference: ParticleReference) {
        self.0.push(particle_reference);
    }
}
impl Field for NGravity {
    fn coupled_particles(&self) -> &[ParticleReference] {
        &self.0
    }
    fn interaction_type(&self) -> InteractionType {
        InteractionType::ParticleParticle
    }
    fn particle_to_particle(&self, particle1: &Particle, particle2: &Particle) -> ParticleAction {
        let radial = particle1.pos - particle2.pos;

        if radial.mag_squared() > 0_f64 {
            ParticleAction::new()
                .force(radial * -(self.1 * particle1.mass * particle2.mass) / radial.mag().powi(3))
        } else {
            ParticleAction::new()
        }
    }
}

//---------------------------------------------------------------------------------------------------//

pub struct DistanceConstraint([ParticleReference; 2], f64);
impl DistanceConstraint {
    pub fn new(linked_particles: [ParticleReference; 2], dist: f64) -> DistanceConstraint {
        DistanceConstraint(linked_particles, dist)
    }
}
impl Field for DistanceConstraint {
    fn coupled_particles(&self) -> &[ParticleReference] {
        &self.0
    }
    fn interaction_type(&self) -> InteractionType {
        InteractionType::ParticleParticle
    }
    fn is_constraint(&self) -> bool {
        true
    }
    fn particle_to_particle(&self, particle1: &Particle, particle2: &Particle) -> ParticleAction {
        let radial = particle1.pos - particle2.pos;
        let dist = radial.mag();
        let correction = self.1 - dist;

        let displacement =
            (radial / dist) * (correction / (particle1.inverse_mass() + particle2.inverse_mass()));

        ParticleAction::new().displacement(displacement)
    }
}

//---------------------------------------------------------------------------------------------------//
