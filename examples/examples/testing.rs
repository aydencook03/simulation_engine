use engine::builtins::*;
use engine::particle::Particle;
use engine::system::System;
use rendering::particle_2d_renderer::Particle2DRenderer;

fn main() {
    let mut system = System::new();
    let window = Particle2DRenderer::new();

    let center = system.add_particle(Particle::new().mass(0.0));
    let mass1 = system.add_particle(Particle::new().pos_xyz(10.0, 250.0, 0.0));
    let mass2 = system.add_particle(
        Particle::new()
            .pos_xyz(350.0, 100.0, 0.0)
            .radius(5.0)
            .mass(2.0),
    );

    let mut gravity = Gravity::new(200.0);
    gravity.add_particle(mass1);
    gravity.add_particle(mass2);

    let dist1 = DistanceConstraint::new([center, mass1], 250.0);
    let dist2 = DistanceConstraint::new([mass1, mass2], (2_f64 * 100_f64.powi(2)).sqrt());

    system.add_field(gravity);
    system.add_field(dist1);
    //system.add_field(dist2);

    window.run(system);
}
