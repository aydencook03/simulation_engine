pub mod algorithms;
pub mod collision;
pub mod constraint;
pub mod interaction;
pub mod math;
pub mod particle;
pub mod system;
// optional feature: pub mod serialization
// optional feature: parallelization or gpu acceleration

pub mod prelude {
    pub use crate::{
        constraint::{builtin_constraints as Constraints, Constraint},
        interaction::{builtin_interactions as Interactions, Interaction},
        math::Vec3,
        particle::Particle,
        system::System,
    };
}
