use engine::prelude::*;
use rendering::particle_2d_renderer::Particle2DRenderer;

fn main() {
    let mut system = System::new();
    let window = Particle2DRenderer::new(None);

    system.add_particle(Particle::new().mass(50.0).vel_xyz(-10.0, 0.0, 14.0));
    system.add_particle(
        Particle::new()
            .pos_xyz(100.0, 100.0, 100.0)
            .vel_xyz(0.0, 0.0, -70.0),
    );
    system.add_particle(
        Particle::new()
            .mass(5.0)
            .pos_xyz(0.0, 0.0, 100.0)
            .vel_xyz(100.0, 0.0, 0.0),
    );

    let gravity = Interactions::Gravity::new(40000.0).with_particles(&system.all_particles());
    system.add_interaction(gravity);

    window.run(system);
}
