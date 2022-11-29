pub mod algorithms;
pub mod collision;
pub mod constraint;
pub mod interaction;
pub mod math;
pub mod particle;
pub mod system;

pub mod prelude {
    pub use crate::{
        constraint::{builtin_constraints as Constraints, Constraint},
        interaction::builtin_interactions as Interactions,
        math::{Matrix3, Vec3, PI},
        particle::Particle,
        system::System,
    };
}
