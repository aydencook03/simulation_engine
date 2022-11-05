use engine::prelude::*;
use rendering::particle_2d_renderer::Particle2DRenderer;

fn main() {
    let mut system = System::new();
    let mut window = Particle2DRenderer::new();
    window.scale.physics_dt = 1.0 / 180.0;

    let center = system.add_particle(Particle::new().mass(0.0));
    let mass1 = system.add_particle(Particle::new().pos_xyz(0.0, 250.0, 0.0).mass(15.0));
    let mass2 = system.add_particle(Particle::new().pos_xyz(100.0, 250.0, 0.0).mass(8.0));
    let mass3 = system.add_particle(Particle::new().pos_xyz(200.0, 250.0, 0.0).mass(2.0));

    let mut gravity = Fields::Falling::new(200.0);
    gravity.add_particles(&system.all_particles());

    let dist1 = Constraints::Distance::new([center, mass1], 250.);
    let dist2 = Constraints::Distance::new([mass1, mass2], 100.);
    let dist3 = Constraints::Distance::new([mass2, mass3], 100.);

    system.add_field(gravity);
    system.add_constraint(dist1);
    system.add_constraint(dist2);
    system.add_constraint(dist3);
    system.static_constraint_pass(1);

    window.run(system);
}
