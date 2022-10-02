use crate::field::Field;
use crate::particle::{Particle, ParticleReference};

//---------------------------------------------------------------------------------------------------//

#[derive(Default)]
pub struct System {
    pub time: f64,
    pub running: bool,
    pub substeps: u32,

    pub particles: Vec<Particle>,
    pub fields: Vec<Box<dyn Field>>,

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

        ParticleReference {
            id,
            index: self.particles.len() - 1,
        }
    }

    pub fn add_field(&mut self, field: impl Field + 'static) -> usize {
        self.fields.push(Box::new(field));
        self.fields.len() - 1
    }

    pub fn all_particles(&self) -> Vec<ParticleReference> {
        let mut particles = Vec::new();
        for index in 0..self.particles.len() {
            particles.push(ParticleReference {
                id: self.particles[index].id,
                index,
            });
        }
        particles
    }

    //--------------------------------------------------------------------//

    pub fn step_forward(&mut self, dt: f64) {
        if self.running {
            for _ in 0..self.substeps {
                let sub_dt = dt / (self.substeps as f64);

                for particle in &mut self.particles {
                    particle.integrate(sub_dt);
                    particle.clear_force();
                }

                <dyn Field>::handle_fields(&mut self.fields, &mut self.particles, sub_dt);
            }
            self.time += dt;
        }
    }
}

//---------------------------------------------------------------------------------------------------//
