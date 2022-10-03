
//---------------------------------------------------------------------------------------------------//

#[derive(Copy, Clone)]
pub struct Particle3DRenderer {
    pub physics_dt: f64,
    pub bg_color: [u8; 4],
    pub starting_size: [u32; 2],
}

impl Particle3DRenderer {
    /// Creates a default window.
    pub fn new() -> Particle3DRenderer {
        Particle3DRenderer {
            physics_dt: 1.0 / 120.0,
            bg_color: crate::colors::GREY,
            starting_size: [1000, 1000]
        }
    }
}

//---------------------------------------------------------------------------------------------------//