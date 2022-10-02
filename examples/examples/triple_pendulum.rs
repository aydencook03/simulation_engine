use engine::prelude::*;
use rendering::particle_2d_renderer::Particle2DRenderer;

fn main() {
    let mut system = System::new();
    let window = Particle2DRenderer::new();

    let center = system.add_particle(Particle::new().mass(0.0));
    let mass1 = system.add_particle(Particle::new().pos_xyz(0.0, 250.0, 0.0).mass(15.0));
    let mass2 = system.add_particle(
        Particle::new()
            .pos_xyz(100.0, 250.0, 0.0)
            .radius(8.0)
            .mass(8.0),
    );
    let mass3 = system.add_particle(
        Particle::new()
            .pos_xyz(200.0, 250.0, 0.0)
            .radius(4.0)
            .mass(2.0),
    );

    let mut gravity = Gravity::new(200.0);
    gravity.add_particles(&system.all_particles());

    let mut dist1 = DistanceConstraint::new(250.0);
    dist1.add_particles(&[center, mass1]);
    let mut dist2 = DistanceConstraint::new(100.0);
    dist2.add_particles(&[mass1, mass2]);
    let mut dist3 = DistanceConstraint::new(100.0);
    dist3.add_particles(&[mass2, mass3]);

    system.add_field(gravity);
    system.add_field(dist1);
    system.add_field(dist2);
    system.add_field(dist3);

    window.run(system);
}
