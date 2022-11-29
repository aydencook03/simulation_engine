use crate::{
    interaction::Interaction,
    math::Vec3,
    particle::{Particle, ParticleReference},
};

//---------------------------------------------------------------------------------------------------//

pub struct FieldForceParameters {
    coupled_particles: Vec<ParticleReference>,
    field: Box<dyn FieldForce>,
}

pub trait FieldForce {
    fn particle_to_field(&mut self, _particle: &Particle) {}

    fn integrate(&mut self, _dt: f64) {}

    fn force_on_particle(&self, _particle: &Particle) -> Option<Vec3> {
        None
    }

    fn clear(&mut self) {}
}

impl FieldForceParameters {
    pub fn new(field: impl FieldForce + 'static) -> FieldForceParameters {
        FieldForceParameters {
            coupled_particles: Vec::new(),
            field: Box::new(field),
        }
    }

    pub fn with_particle(mut self, reference: ParticleReference) -> FieldForceParameters {
        self.coupled_particles.push(reference);
        self
    }

    pub fn with_particles(mut self, references: &[ParticleReference]) -> FieldForceParameters {
        for reference in references {
            self.coupled_particles.push(*reference);
        }
        self
    }
}

impl Interaction for FieldForceParameters {
    fn handle(&mut self, particle_source: &mut [Particle], dt: f64) {
        // ready field for fresh update
        self.field.clear();

        // particles -> act on field
        for reference in &self.coupled_particles.to_owned() {
            // need to find a way around the ".to_owned()"
            self.field.particle_to_field(reference.get(particle_source));
        }

        // field dynamics
        self.field.integrate(dt);

        // field -> act on particles
        for reference in &self.coupled_particles {
            let particle = reference.get_mut(particle_source);
            if let Some(force) = self.field.force_on_particle(particle) {
                particle.add_force(force, particle.pos);
            }
        }
    }
}
