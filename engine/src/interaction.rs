use crate::{
    math::{Vec3, Point3},
    particle::{Particle, ParticleReference},
};

//---------------------------------------------------------------------------------------------------//

pub struct Interaction {
    coupled_particles: Vec<ParticleReference>,
    interaction_type: InteractionType,
}

pub enum InteractionType {
    Generic(Box<dyn GenericInteraction>),
    FieldParticle(Box<dyn FieldParticleForce>),
    PairWiseForce(Box<dyn PairWiseForce>),
    SimpleForce(Box<dyn SimpleForce>),
    Sph(SphParameters),
    Mpm(Box<dyn MpmInteraction>),
}

impl Interaction {
    pub fn new(interaction_type: InteractionType) -> Interaction {
        Interaction {
            coupled_particles: Vec::new(),
            interaction_type,
        }
    }

    pub fn add_particle(&mut self, reference: ParticleReference) {
        self.coupled_particles.push(reference);
    }

    pub fn add_particles(&mut self, references: &[ParticleReference]) {
        for reference in references {
            self.coupled_particles.push(*reference);
        }
    }

    pub fn handle(&mut self, particle_source: &mut [Particle], dt: f64) {
        match &mut self.interaction_type {
            InteractionType::Generic(inter) => inter.interaction(particle_source, dt),
            InteractionType::FieldParticle(inter) => {
                // ready field for fresh update
                inter.clear();

                // particles -> act on field
                for reference in &self.coupled_particles.to_owned() {
                    // need to find a way around the ".to_owned()"
                    inter.particle_to_field(reference.get(particle_source));
                }

                // field dynamics
                inter.integrate(dt);

                // field -> act on particles
                for reference in &self.coupled_particles {
                    let particle = reference.get_mut(particle_source);
                    if let Some(force) = inter.force_on_particle(particle) {
                        particle.add_force(force, particle.pos);
                    }
                }
            }
            InteractionType::PairWiseForce(inter) => {
                for (index, ref1) in self.coupled_particles.iter().enumerate() {
                    for ref2 in &self.coupled_particles[(index + 1)..] {
                        if let Some(force) =
                            inter.force(ref1.get(particle_source), ref2.get(particle_source))
                        {
                            let p1 = ref1.get_mut(particle_source);
                            p1.add_force(force, p1.pos);
                            let p2 = ref2.get_mut(particle_source);
                            p2.add_force(-force, p2.pos);
                        }
                    }
                }
            }
            InteractionType::SimpleForce(inter) => {
                for reference in &self.coupled_particles {
                    if let Some(force) = inter.force(reference.get(particle_source)) {
                        let particle = reference.get_mut(particle_source);
                        particle.add_force(force, particle.pos);
                    }
                }
            }
            InteractionType::Sph(_inter) => todo!(),
            InteractionType::Mpm(_inter) => todo!(),
        }
    }
}

//---------------------------------------------------------------------------------------------------//

pub trait GenericInteraction {
    fn interaction(&mut self, particle_source: &mut [Particle], dt: f64);
}

pub trait FieldParticleForce {
    fn particle_to_field(&mut self, _particle: &Particle) {}

    fn integrate(&mut self, _dt: f64) {}

    fn force_on_particle(&self, _particle: &Particle) -> Option<Vec3> {
        None
    }

    fn clear(&mut self) {}
}

pub trait PairWiseForce {
    fn force(&self, _particle1: &Particle, _particle2: &Particle) -> Option<Vec3>;
}

pub trait SimpleForce {
    fn force(&self, _particle: &Particle) -> Option<Vec3>;
}

//--------------------------------------------------------------------//

pub struct SphParameters {
    kernel: Box<dyn SphKernel>,
    smoothing_radius: Option<f64>,
}

impl SphParameters {
    
}

pub trait SphKernel {
    fn w(&self, r: Point3, h: f64) -> f64;
    fn grad_w(&self, r: Point3, h: f64) -> Vec3;
    fn div_w(&self, r: Point3, h: f64) -> f64;
    fn laplace_w(&self, r: Point3, h: f64) -> f64;
}

//--------------------------------------------------------------------//

pub trait MpmInteraction {}

//---------------------------------------------------------------------------------------------------//

/* future better version?
pub trait Interaction {
    fn handle(&mut self, particle_source: &mut [Particle], dt: f64);
    fn interaction_energy(&self, particle_source: &[Particle]) -> f64;
}

impl<T: GenericInteraction> Interaction for T {
    fn handle(&mut self, particle_source: &mut [Particle], dt: f64) {
        self.interaction(particle_source, dt);
    }
}

impl<T: FieldParticleInteraction> Interaction for T {
    fn handle(&mut self, particle_source: &mut [Particle], dt: f64) {
        todo!()
    }

    fn interaction_energy(&self, particle_source: &[Particle]) -> f64 {
        todo!()
    }
} */

//---------------------------------------------------------------------------------------------------//

pub mod builtin_interactions {
    use crate::{
        interaction::{Interaction, InteractionType, PairWiseForce, SimpleForce},
        math::Vec3,
        particle::Particle,
    };

    //--------------------------------------------------------------------//
    pub struct ConstantForce(Vec3);

    impl ConstantForce {
        pub fn new(force: Vec3) -> ConstantForce {
            ConstantForce(force)
        }

        pub fn build(self) -> Interaction {
            Interaction::new(InteractionType::SimpleForce(Box::new(self)))
        }
    }

    impl SimpleForce for ConstantForce {
        fn force(&self, _particle: &Particle) -> Option<Vec3> {
            Some(self.0)
        }
    }

    //--------------------------------------------------------------------//

    pub struct Falling(f64, f64);

    impl Falling {
        pub fn new(g: f64) -> Falling {
            Falling(g, 0.0)
        }

        pub fn ground_reference(mut self, height: f64) -> Falling {
            self.1 = height;
            self
        }

        pub fn build(self) -> Interaction {
            Interaction::new(InteractionType::SimpleForce(Box::new(self)))
        }
    }

    impl SimpleForce for Falling {
        fn force(&self, particle: &Particle) -> Option<Vec3> {
            Some(Vec3::new(0.0, -particle.mass * self.0, 0.0))
        }
    }

    //--------------------------------------------------------------------//

    pub struct Gravity(f64);

    impl Gravity {
        pub fn new(gravitational_constant: f64) -> Gravity {
            Gravity(gravitational_constant)
        }

        pub fn build(self) -> Interaction {
            Interaction::new(InteractionType::PairWiseForce(Box::new(self)))
        }
    }

    impl PairWiseForce for Gravity {
        fn force(&self, particle1: &Particle, particle2: &Particle) -> Option<Vec3> {
            let radial = particle2.pos - particle1.pos;
            let dist = radial.mag();

            Some(((self.0 * particle1.mass * particle2.mass) / dist.powi(3)) * radial)
        }
    }

    //--------------------------------------------------------------------//

    pub struct ElectroStatic(f64);

    impl ElectroStatic {
        pub fn new(electrostatic_constant: f64) -> ElectroStatic {
            ElectroStatic(electrostatic_constant)
        }

        pub fn build(self) -> Interaction {
            Interaction::new(InteractionType::PairWiseForce(Box::new(self)))
        }
    }

    impl PairWiseForce for ElectroStatic {
        fn force(&self, particle1: &Particle, particle2: &Particle) -> Option<Vec3> {
            let radial = particle2.pos - particle1.pos;
            let dist = radial.mag();

            Some(-((self.0 * particle1.charge * particle2.charge) / dist.powi(3)) * radial)
        }
    }

    //--------------------------------------------------------------------//

    pub struct LennardJones {
        dispersion_energy: f64,
        collision_radius: f64,
        repulsion: f64,
        attraction: f64,
    }

    impl LennardJones {
        pub fn new(dispersion_energy: f64, collision_radius: f64) -> LennardJones {
            LennardJones {
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

        pub fn build(self) -> Interaction {
            Interaction::new(InteractionType::PairWiseForce(Box::new(self)))
        }
    }

    impl PairWiseForce for LennardJones {
        fn force(&self, particle1: &Particle, particle2: &Particle) -> Option<Vec3> {
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
    }
}

//---------------------------------------------------------------------------------------------------//
