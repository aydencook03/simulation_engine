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

pub struct NGravity(Vec<ParticleReference>, Vec<(f64, Vec3)>, f64);
impl NGravity {
    pub fn new(gravitational_constant: f64) -> NGravity {
        NGravity(Vec::new(), Vec::new(), gravitational_constant)
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
    fn particle_to_field(&mut self, particle: &Particle) {
        self.1.push((particle.mass, particle.pos));
    }
    fn field_to_particle(&self, particle: &Particle) -> ParticleAction {
        let mut total_force = Vec3::zero();
        for attractor in &self.1 {
            let radial = particle.pos - attractor.1;

            if radial.mag_squared() > 0_f64 {
                total_force +=
                    radial * -(self.2 * attractor.0 * particle.mass) / radial.mag().powi(3);
            }
        }

        ParticleAction::new().force(total_force)
    }
    fn clear(&mut self) {
        self.1.clear();
    }
}

//---------------------------------------------------------------------------------------------------//

pub struct DistanceConstraint([ParticleReference; 2], Vec<(Vec3, f64, u32)>, f64);
impl DistanceConstraint {
    pub fn new(particles: [ParticleReference; 2], dist: f64) -> DistanceConstraint {
        DistanceConstraint(particles, Vec::new(), dist)
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
    fn particle_to_field(&mut self, particle: &Particle) {
        self.1
            .push((particle.pos, particle.inverse_mass(), particle.id));
    }
    fn field_to_particle(&self, particle: &Particle) -> ParticleAction {
        let mut displacement = Vec3::zero();
        for (pos, inv_mass, id) in &self.1 {
            if particle.id != *id {
                let radial = particle.pos - *pos;
                let dist = radial.mag();
                let correction = self.2 - dist;

                displacement =
                    (radial / dist) * (correction / (particle.inverse_mass() + inv_mass));
            }
        }
        ParticleAction::new().displacement(displacement)
    }
    fn clear(&mut self) {
        self.1.clear();
    }
}

//---------------------------------------------------------------------------------------------------//
