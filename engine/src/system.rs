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
            substeps: 5,
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

    pub fn add_field(&mut self, field: impl Field + 'static) {
        self.fields.push(Box::new(field));
    }

    //--------------------------------------------------------------------//

    pub fn step_forward(&mut self, dt: f64) {
        if self.running {
            for _ in 0..self.substeps {
                let sub_dt = dt / (self.substeps as f64);
                // detect collisions / gather neighbors
                // temperature (radiation, conduction, advection, thermal-expansion, friction/collision heating)

                for field in &mut self.fields {
                    field.handle(&mut self.particles, sub_dt);
                }

                for particle in &mut self.particles {
                    particle.integrate(sub_dt);
                    particle.vel_from_prev_pos();
                    particle.clear();
                }

                self.time += dt;

                /* for _ in 0..self.xpbd_substeps {
                    let xpbd_dt = sub_dt / (self.xpbd_substeps as f64);
                    for particle in &mut self.particles {
                        particle.integrate(xpbd_dt);
                    }
                    // project constraints
                    for particle in &mut self.particles {
                        particle.vel_from_prev_pos();
                    }
                }

                self.time += dt;
                for particle in &mut self.particles {
                    particle.clear();
                }
                // clear broken constraints */
            }
        }
    }
}

//---------------------------------------------------------------------------------------------------//
