use crate::constraint::Constraint;
use crate::field::Field;
use crate::particle::{Particle, ParticleReference};
use crate::vec3::Vec3;

//---------------------------------------------------------------------------------------------------//

#[derive(Default)]
pub struct System {
    pub time: f64,
    pub running: bool,
    pub substeps: u32,

    pub particles: Vec<Particle>,
    pub fields: Vec<Box<dyn Field>>,
    pub constraints: Vec<Box<dyn Constraint>>,

    pub id_counter: u32,
}

//---------------------------------------------------------------------------------------------------//

impl System {
    pub fn new() -> System {
        System {
            running: true,
            substeps: 20,
            ..Default::default()
        }
    }

    pub fn add_particle(&mut self, particle: Particle) -> ParticleReference {
        let id = self.id_counter;
        self.particles.push(particle.id(id));
        self.id_counter += 1;

        ParticleReference::new(id, self.particles.len() - 1)
    }

    pub fn add_particles(&mut self, particles: Vec<Particle>) -> Vec<ParticleReference> {
        let mut references = Vec::new();
        for particle in particles {
            let id = self.id_counter;
            self.particles.push(particle.id(id));
            self.id_counter += 1;
            references.push(ParticleReference::new(id, self.particles.len() - 1));
        }
        references
    }

    pub fn add_field(&mut self, field: impl Field + 'static) -> usize {
        self.fields.push(Box::new(field));
        self.fields.len() - 1
    }

    pub fn add_constraint(&mut self, constraint: impl Constraint + 'static) -> usize {
        self.constraints.push(Box::new(constraint));
        self.constraints.len() - 1
    }

    //--------------------------------------------------------------------//

    pub fn all_particles(&self) -> Vec<ParticleReference> {
        let mut references = Vec::new();
        let mut index = 0;
        for particle in &self.particles {
            references.push(ParticleReference::new(particle.id, index));
            index += 1;
        }
        references
    }

    pub fn particles_in_group(&self, group: u32) -> Vec<ParticleReference> {
        let mut references = Vec::new();
        let mut index = 0;
        for particle in &self.particles {
            if particle.group == group {
                references.push(ParticleReference::new(particle.id, index));
            }
            index += 1;
        }
        references
    }

    //--------------------------------------------------------------------//

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

    //--------------------------------------------------------------------//

    pub fn static_constraint_pass(&mut self, iterations: u32) {
        for _ in 0..iterations {
            for constraint in &mut self.constraints {
                constraint.project(&mut self.particles, core::f64::MIN_POSITIVE);
            }
        }
    }

    pub fn step_forward(&mut self, dt: f64) {
        if self.running && dt != 0_f64 {
            self.debug_angular_momentum();
            for _ in 0..self.substeps {
                let sub_dt = dt / (self.substeps as f64);

                for field in &mut self.fields {
                    field.handle(&mut self.particles, sub_dt);
                }

                for particle in &mut self.particles {
                    particle.integrate(sub_dt);
                    particle.forces.clear();
                }

                for constraint in &mut self.constraints {
                    constraint.project(&mut self.particles, sub_dt);
                }

                for particle in &mut self.particles {
                    particle.vel = (particle.pos - particle.prev_pos) / sub_dt;
                }
            }
            self.time += dt;
        }
    }
}

//---------------------------------------------------------------------------------------------------//
