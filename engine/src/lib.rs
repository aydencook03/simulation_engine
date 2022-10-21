pub mod algorithms;
pub mod builtin_fields;
pub mod constraint;
pub mod field;
pub mod particle;
pub mod system;
pub mod vec3;
// optional feature: pub mod serialization
// optional feature: parallelization or gpu acceleration

pub mod prelude {
    pub use crate::{constraint::Constraint, builtin_fields::*, field::Field, particle::Particle, system::System, vec3::Vec3};
}
