pub mod algorithms;
pub mod builtins;
pub mod field;
pub mod particle;
pub mod system;
pub mod vec3;
// optional feature: pub mod serialization

pub mod prelude {
    pub use crate::{builtins::*, particle::Particle, system::System, vec3::Vec3};
}
