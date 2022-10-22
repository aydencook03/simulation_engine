pub mod algorithms;
pub mod constraint;
pub mod field;
pub mod particle;
pub mod system;
pub mod vec3;
// optional feature: pub mod serialization
// optional feature: parallelization or gpu acceleration

pub mod prelude {
    pub use crate::{
        field::builtin_fields as Fields, constraint::Constraint, field::Field, particle::Particle,
        system::System, vec3::Vec3,
    };
}
