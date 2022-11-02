pub mod algorithms;
pub mod collision;
pub mod constraint;
pub mod field;
pub mod particle;
pub mod system;
pub mod vec3;
// optional feature: pub mod serialization
// optional feature: parallelization or gpu acceleration

pub mod prelude {
    pub use crate::{
        constraint::builtin_constraints as Constraints, constraint::Constraint,
        field::builtin_fields as Fields, field::Field, particle::Particle, system::System,
        vec3::Tensor, vec3::Vec3,
    };
}
