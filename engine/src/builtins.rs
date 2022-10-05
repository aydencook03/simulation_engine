use crate::{
    field::{CoupledParticles, Field, InteractionType, ParticleAction},
    particle::{Particle, Vec3},
};

//---------------------------------------------------------------------------------------------------//

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

//---------------------------------------------------------------------------------------------------//

pub struct Gravity(CoupledParticles, f64);

impl Gravity {
    pub fn new(g: f64) -> Gravity {
        Gravity(CoupledParticles::new(), g)
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
        InteractionType::FieldParticle
    }
    fn field_to_particle(&self, particle: &Particle) -> ParticleAction {
        ParticleAction::new().force(Vec3::new(0.0, -particle.mass * self.1, 0.0))
    }
}

//---------------------------------------------------------------------------------------------------//

pub struct NGravity(CoupledParticles, f64, f64);

impl NGravity {
    pub fn new(gravitational_constant: f64, softening_parameter: f64) -> NGravity {
        NGravity(
            CoupledParticles::new(),
            gravitational_constant,
            softening_parameter,
        )
    }
}

impl Field for NGravity {
    fn coupled_particles(&self) -> &CoupledParticles {
        &self.0
    }
    fn coupled_particles_mut(&mut self) -> &mut CoupledParticles {
        &mut self.0
    }
    fn interaction_type(&self) -> InteractionType {
        InteractionType::ParticleParticle
    }
    fn particle_to_particle(&self, particle1: &Particle, particle2: &Particle) -> ParticleAction {
        let radial = particle1.pos - particle2.pos;
        let dist_sqr = radial.mag_squared();

        ParticleAction::new().force(
            -(self.1 * particle1.mass * particle2.mass)
                / (dist_sqr + self.2.powi(2)).powi(3).sqrt()
                * radial,
        )
    }
}

//---------------------------------------------------------------------------------------------------//

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
    fn particle_to_particle(&self, particle1: &Particle, particle2: &Particle) -> ParticleAction {
        let radial = particle1.pos - particle2.pos;
        let dist_sqr = radial.mag_squared();
        let bond_6 = if let Some(length) = self.2 {
            length.powi(6)
        } else {
            (particle1.radius + particle2.radius).powi(6)
        };
        let bond_12 = bond_6.powi(2);

        let denom = dist_sqr + self.3.powi(2);
        ParticleAction::new()
            .force((-12_f64 * self.1 * (bond_6 / denom.powi(4) - bond_12 / denom.powi(7))) * radial)
    }
}

//---------------------------------------------------------------------------------------------------//

pub struct BoxBound(CoupledParticles, Vec3, Vec3);

impl BoxBound {
    pub fn new(back_bottom_left: Vec3, front_top_right: Vec3) -> BoxBound {
        BoxBound(CoupledParticles::new(), back_bottom_left, front_top_right)
    }
}

impl Field for BoxBound {
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
        let mut unsatisfied = false;
        let mut displacement = Vec3::zero();
        let mut impulse = Vec3::zero();

        if (particle.pos.x - particle.radius) < self.1.x {
            unsatisfied = true;
            displacement.x += self.1.x - (particle.pos.x - particle.radius);
            if particle.vel.x < 0.0 {
                impulse.x += -2.0 * particle.vel.x * particle.mass;
            }
        } else if (particle.pos.x + particle.radius) > self.2.x {
            unsatisfied = true;
            displacement.x -= (particle.pos.x + particle.radius) - self.2.x;
            if particle.vel.x > 0.0 {
                impulse.x += -2.0 * particle.vel.x * particle.mass;
            }
        }

        if (particle.pos.y - particle.radius) < self.1.y {
            unsatisfied = true;
            displacement.y += self.1.y - (particle.pos.y - particle.radius);
            if particle.vel.y < 0.0 {
                impulse.y += -2.0 * particle.vel.y * particle.mass;
            }
        } else if (particle.pos.y + particle.radius) > self.2.y {
            unsatisfied = true;
            displacement.y -= (particle.pos.y + particle.radius) - self.2.y;
            if particle.vel.y > 0.0 {
                impulse.y += -2.0 * particle.vel.y * particle.mass;
            }
        }

        if (particle.pos.z - particle.radius) < self.1.z {
            unsatisfied = true;
            displacement.z += self.1.z - (particle.pos.z - particle.radius);
            if particle.vel.z < 0.0 {
                impulse.z += -2.0 * particle.vel.z * particle.mass;
            }
        } else if (particle.pos.z + particle.radius) > self.2.z {
            unsatisfied = true;
            displacement.z -= (particle.pos.z + particle.radius) - self.2.z;
            if particle.vel.z > 0.0 {
                impulse.z += -2.0 * particle.vel.z * particle.mass;
            }
        }

        if unsatisfied {
            ParticleAction::new()
                .displacement(displacement)
                .impulse(impulse)
        } else {
            ParticleAction::new()
        }
    }
}

//---------------------------------------------------------------------------------------------------//

pub struct DistanceConstraint {
    particles: CoupledParticles,
    distance: f64,
    compliance: f64,
}

impl DistanceConstraint {
    pub fn new(distance: f64) -> DistanceConstraint {
        DistanceConstraint {
            particles: CoupledParticles::new(),
            distance,
            compliance: 0.0,
        }
    }

    pub fn compliance(mut self, compliance: f64) -> DistanceConstraint {
        self.compliance = compliance;
        self
    }
}

impl Field for DistanceConstraint {
    fn coupled_particles(&self) -> &CoupledParticles {
        &self.particles
    }
    fn coupled_particles_mut(&mut self) -> &mut CoupledParticles {
        &mut self.particles
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
        let correction = self.distance - dist;
        let inv_mass = particle1.inverse_mass();

        let displacement =
            inv_mass * correction / (inv_mass + particle2.inverse_mass()) * (radial / dist);

        ParticleAction::new().displacement(displacement)
    }
}

//---------------------------------------------------------------------------------------------------//

pub struct MinDistanceConstraint(CoupledParticles, f64);

impl MinDistanceConstraint {
    pub fn new(minimum_dist: f64) -> MinDistanceConstraint {
        MinDistanceConstraint(CoupledParticles::new(), minimum_dist)
    }
}

impl Field for MinDistanceConstraint {
    fn coupled_particles(&self) -> &CoupledParticles {
        &self.0
    }
    fn coupled_particles_mut(&mut self) -> &mut CoupledParticles {
        &mut self.0
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

        if dist < self.1 {
            let correction = self.1 - dist;
            let inv_mass = particle1.inverse_mass();

            let displacement =
                (radial / dist) * inv_mass * (correction / (inv_mass + particle2.inverse_mass()));

            ParticleAction::new().displacement(displacement)
        } else {
            ParticleAction::new()
        }
    }
}

//---------------------------------------------------------------------------------------------------//

pub struct NoOverlapConstraint(CoupledParticles);

impl NoOverlapConstraint {
    pub fn new() -> NoOverlapConstraint {
        NoOverlapConstraint(CoupledParticles::new())
    }
}

impl Field for NoOverlapConstraint {
    fn coupled_particles(&self) -> &CoupledParticles {
        &self.0
    }
    fn coupled_particles_mut(&mut self) -> &mut CoupledParticles {
        &mut self.0
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
        let radii_sum = particle1.radius + particle2.radius;

        if dist < radii_sum {
            let overlap = radii_sum - dist;
            let inv_mass = particle1.inverse_mass();

            let displacement =
                (radial / dist) * inv_mass * (overlap / (inv_mass + particle2.inverse_mass()));

            ParticleAction::new().displacement(displacement)
        } else {
            ParticleAction::new()
        }
    }
}

//---------------------------------------------------------------------------------------------------//

pub struct BoxBoundConstraint(CoupledParticles, Vec3, Vec3);

impl BoxBoundConstraint {
    pub fn new(back_bottom_left: Vec3, front_top_right: Vec3) -> BoxBoundConstraint {
        BoxBoundConstraint(CoupledParticles::new(), back_bottom_left, front_top_right)
    }
}

impl Field for BoxBoundConstraint {
    fn coupled_particles(&self) -> &CoupledParticles {
        &self.0
    }
    fn coupled_particles_mut(&mut self) -> &mut CoupledParticles {
        &mut self.0
    }
    fn interaction_type(&self) -> InteractionType {
        InteractionType::FieldParticle
    }
    fn is_constraint(&self) -> bool {
        true
    }
    fn field_to_particle(&self, particle: &Particle) -> ParticleAction {
        let mut unsatisfied = false;
        let mut displacement = Vec3::zero();

        if (particle.pos.x - particle.radius) < self.1.x {
            unsatisfied = true;
            displacement.x += self.1.x - (particle.pos.x - particle.radius);
        } else if (particle.pos.x + particle.radius) > self.2.x {
            unsatisfied = true;
            displacement.x -= (particle.pos.x + particle.radius) - self.2.x;
        }

        if (particle.pos.y - particle.radius) < self.1.y {
            unsatisfied = true;
            displacement.y += self.1.y - (particle.pos.y - particle.radius);
        } else if (particle.pos.y + particle.radius) > self.2.y {
            unsatisfied = true;
            displacement.y -= (particle.pos.y + particle.radius) - self.2.y;
        }

        if (particle.pos.z - particle.radius) < self.1.z {
            unsatisfied = true;
            displacement.z += self.1.z - (particle.pos.z - particle.radius);
        } else if (particle.pos.z + particle.radius) > self.2.z {
            unsatisfied = true;
            displacement.z -= (particle.pos.z + particle.radius) - self.2.z;
        }

        if unsatisfied {
            ParticleAction::new().displacement(displacement)
        } else {
            ParticleAction::new()
        }
    }
}

//---------------------------------------------------------------------------------------------------//
