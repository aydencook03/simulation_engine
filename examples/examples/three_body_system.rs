use engine::prelude::*;
use rendering::particle_2d_renderer::Particle2DRenderer;

fn main() {
    let mut system = System::new();
    let window = Particle2DRenderer::new();

    let mut gravity = NGravity::new(20000.0, None);
    system.add_particle(Particle::new().radius(20.0).mass(50.0));
    system.add_particle(
        Particle::new()
            .pos_xyz(100.0, 100.0, 100.0)
            .vel_xyz(0.0, 0.0, -70.0),
    );
    system.add_particle(
        Particle::new()
            .radius(5.0)
            .mass(5.0)
            .pos_xyz(0.0, 0.0, 100.0)
            .vel_xyz(100.0, 0.0, 0.0),
    );

    gravity.add_particles(&system.all_particles());
    system.add_field(gravity);

    window.run(system);
}
