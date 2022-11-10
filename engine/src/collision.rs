use crate::{
    math::Point3,
    particle::{Particle, ParticleReference},
};

//---------------------------------------------------------------------------------------------------//

pub trait CollisionDetector {
    fn contact_pairs(
        &mut self,
        particles: &[Particle],
    ) -> Vec<(ParticleReference, ParticleReference, Point3)>;
}

//---------------------------------------------------------------------------------------------------//

/* struct AABB {
    min: Point3,
    max: Point3,
} */
