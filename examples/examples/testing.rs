use engine::prelude::*;
use rendering::particle_2d_renderer::Particle2DRenderer;

fn main() {
    let system = System::new();
    let window = Particle2DRenderer::new();

    window.run(system);
}
