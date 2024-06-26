use crate::constraint::Constraint;
use crate::interaction::Interaction;
use crate::math::Vec3;
use crate::particle::{Particle, ParticleReference};

//---------------------------------------------------------------------------------------------------//

#[derive(Default)]
pub struct System {
    pub time: f64,
    pub running: bool,
    pub substeps: u32,

    pub particle_radius: f64,

    pub particles: Vec<Particle>,
    pub interactions: Vec<Box<dyn Interaction>>,
    pub constraints: Vec<Box<dyn Constraint>>,

    pub id_counter: u32,
}

//---------------------------------------------------------------------------------------------------//

impl System {
    pub fn new() -> System {
        System {
            running: true,
            substeps: 20,
            particle_radius: 5.0,
            ..Default::default()
        }
    }

    //--------------------------------------------------------------------//
    // adder methods

    pub fn add_particle(&mut self, particle: Particle) -> ParticleReference {
        let id = self.id_counter;
        self.particles.push(particle.id(id));
        self.id_counter += 1;

        ParticleReference::new(id, self.particles.len() - 1)
    }

    pub fn add_particles(&mut self, particles: Vec<Particle>) -> Vec<ParticleReference> {
        let mut references = Vec::new();
        for particle in particles {
            references.push(self.add_particle(particle));
        }
        references
    }

    pub fn add_interaction(&mut self, interaction: impl Interaction + 'static) -> usize {
        self.interactions.push(Box::new(interaction));
        self.interactions.len() - 1
    }

    pub fn add_constraint(&mut self, constraint: impl Constraint + 'static) -> usize {
        self.constraints.push(Box::new(constraint));
        self.constraints.len() - 1
    }

    //--------------------------------------------------------------------//
    // methods for retrieving particle references

    pub fn all_particles(&self) -> Vec<ParticleReference> {
        let mut references = Vec::new();
        for (index, particle) in self.particles.iter().enumerate() {
            references.push(ParticleReference::new(particle.id, index));
        }
        references
    }

    pub fn particles_in_group(&self, group: u32) -> Vec<ParticleReference> {
        let mut references = Vec::new();
        for (index, particle) in self.particles.iter().enumerate() {
            if particle.group == group {
                references.push(ParticleReference::new(particle.id, index));
            }
        }
        references
    }

    //--------------------------------------------------------------------//
    // debugging

    pub fn debug_momentum(&self) {
        let mut momentum = Vec3::zero();
        for particle in &self.particles {
            momentum += particle.mass * particle.vel;
        }
        dbg!(momentum);
    }

    pub fn debug_angular_momentum(&self) {
        let mut angular_momentum = Vec3::zero();
        for particle in &self.particles {
            angular_momentum += particle.pos.cross(particle.mass * particle.vel);
        }
        dbg!(angular_momentum);
    }

    pub fn debug_kinetic_energy(&self) {
        let mut ke = 0.0;
        for particle in &self.particles {
            ke += 0.5 * particle.mass * particle.vel.mag_squared();
        }
        dbg!(ke);
    }

    //--------------------------------------------------------------------//
    // time evolution

    pub fn static_constraint_pass(&mut self, iterations: u32) {
        for _ in 0..iterations {
            for constraint in &mut self.constraints {
                constraint.project(&mut self.particles, core::f64::MAX, true);
            }
        }
    }

    pub fn step_forward(&mut self, dt: f64) {
        if !self.running || dt == 0_f64 {
            return;
        }

        let sub_dt = dt / (self.substeps as f64);
        for _ in 0..self.substeps {
            for interaction in &mut self.interactions {
                interaction.handle(&mut self.particles, sub_dt);
            }

            for particle in &mut self.particles {
                particle.integrate(sub_dt);
                particle.forces.clear();
            }

            for constraint in &mut self.constraints {
                constraint.project(&mut self.particles, sub_dt, false);
            }

            for particle in &mut self.particles {
                particle.update_vel(sub_dt);
            }
        }
        self.time += dt;
    }
}

//---------------------------------------------------------------------------------------------------//
