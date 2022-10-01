use engine::prelude::*;
use rendering::particle_2d_renderer::Particle2DRenderer;

fn main() {
    let mut system = System::new();
    let window = Particle2DRenderer::new();

    let center = system.add_particle(Particle::new().mass(0.0));
    let mass1 = system.add_particle(Particle::new().pos_xyz(20.0, 250.0, 0.0).mass(15.0));
    let mass2 = system.add_particle(
        Particle::new()
            .pos_xyz(100.0, 250.0, 0.0)
            .radius(8.0)
            .mass(8.0),
    );
    let mass3 = system.add_particle(Particle::new().pos_xyz(150.0, 250.0, 0.0).radius(5.0).mass(2.0));

    let mut gravity = Gravity::new(200.0);
    gravity.add_particle(mass1);
    gravity.add_particle(mass2);
    gravity.add_particle(mass3);

    let dist1 = DistanceConstraint::new([center, mass1], 250.0);
    let dist2 = DistanceConstraint::new([mass1, mass2], 100.0);
    let dist3 = DistanceConstraint::new([mass2, mass3], 50.0);

    system.add_field(gravity);
    system.add_field(dist1);
    system.add_field(dist2);
    system.add_field(dist3);

    window.run(system);
}
