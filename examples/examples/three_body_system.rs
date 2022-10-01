use engine::prelude::*;
use rendering::particle_2d_renderer::Particle2DRenderer;

fn main() {
    let mut system = System::new();
    let window = Particle2DRenderer::new();

    let mut gravity = NGravity::new(2000.0, None);
    let mass1 = system.add_particle(Particle::new());

    gravity.add_particle(mass1);

    system.add_field(gravity);

    window.run(system);
}
