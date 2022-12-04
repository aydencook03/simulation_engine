use crate::{
    constraint::xpbd::{Xpbd, XpbdParameters},
    math::{Point3, Vec3},
    particle::{Particle, ParticleReference},
};

//---------------------------------------------------------------------------------------------------//

pub struct Distance([ParticleReference; 2], f64);

impl Distance {
    pub fn new(particles: [ParticleReference; 2], dist: f64) -> XpbdParameters {
        XpbdParameters::new(Distance(particles, dist))
    }
}

impl Xpbd for Distance {
    fn particles(&self) -> &[ParticleReference] {
        &self.0
    }

    fn constraint(&self, particles: &[&Particle]) -> f64 {
        self.1 - (particles[1].pos - particles[0].pos).mag()
    }

    fn gradients(&self, particles: &[&Particle]) -> Vec<Vec3> {
        let norm = (particles[1].pos - particles[0].pos).norm();
        vec![norm, -norm]
    }
}

//--------------------------------------------------------------------//

pub struct NonPenetrate([ParticleReference; 2], f64);

impl NonPenetrate {
    pub fn new(particles: [ParticleReference; 2], collision_distance: f64) -> XpbdParameters {
        XpbdParameters::new(NonPenetrate(particles, collision_distance)).as_inequality()
    }
}

impl Xpbd for NonPenetrate {
    fn particles(&self) -> &[ParticleReference] {
        &self.0
    }

    fn constraint(&self, particles: &[&Particle]) -> f64 {
        (particles[1].pos - particles[0].pos).mag() - self.1
    }

    fn gradients(&self, particles: &[&Particle]) -> Vec<Vec3> {
        let norm = (particles[1].pos - particles[0].pos).norm();
        vec![-norm, norm]
    }
}

//--------------------------------------------------------------------//

pub struct ContactPlane {
    particle: [ParticleReference; 1],
    collision_distance: f64,
    point: Point3,
    normal: Vec3,
}

impl ContactPlane {
    pub fn new(
        particle: ParticleReference,
        collision_distance: f64,
        point: Point3,
        normal: Vec3,
    ) -> XpbdParameters {
        XpbdParameters::new(ContactPlane {
            particle: [particle],
            collision_distance,
            point,
            normal: normal.norm(),
        })
        .as_inequality()
    }
}

impl Xpbd for ContactPlane {
    fn particles(&self) -> &[ParticleReference] {
        &self.particle
    }

    fn constraint(&self, particles: &[&Particle]) -> f64 {
        (particles[0].pos - self.point).dot(self.normal) - self.collision_distance
    }

    fn gradients(&self, _particles: &[&Particle]) -> Vec<Vec3> {
        vec![self.normal]
    }
}

//---------------------------------------------------------------------------------------------------//
