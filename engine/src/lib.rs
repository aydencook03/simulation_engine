pub mod algorithms;
pub mod collision;
pub mod constraint;
pub mod field;
pub mod math;
pub mod particle;
pub mod system;
// optional feature: pub mod serialization
// optional feature: parallelization or gpu acceleration

pub mod prelude {
    pub use crate::{
        constraint::builtin_constraints as Constraints, constraint::Constraint,
        field::builtin_fields as Fields, field::Field, math::Vec3, particle::Particle,
        system::System,
    };
}
