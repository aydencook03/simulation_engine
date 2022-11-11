use engine::prelude::*;
use rendering::particle_2d_renderer::Particle2DRenderer;

fn main() {
    let mut system = System::new();
    let window = Particle2DRenderer::new();

    system.add_particle(Particle::new().pos_xyz(40.0, 0.0, 0.0).charge(-20.0));
    system.add_particle(Particle::new().pos_xyz(-40.0, 0.0, 0.0).charge(20.0));

    let mut electric = Interactions::ElectroStatic::new(100.0).build();
    electric.add_particles(&system.all_particles());
    system.add_interaction(electric);

    window.run(system);
}
