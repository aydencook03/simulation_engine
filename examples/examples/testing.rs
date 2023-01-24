use engine::prelude::*;
use rendering::particle_2d_renderer::Particle2DRenderer;

fn main() {
    let mut system = System::new();
    let mut window = Particle2DRenderer::new(Some(print_ang_momentum));
    window.scale.physics_dt = 1.0 / 240.0;

    system.add_particle(
        Particle::new()
            .pos_xyz(100.0, 0.0, 0.0)
            .vel_xyz(0.0, 5.0, 0.0)
            .mass(50.0),
    );
    system.add_particle(
        Particle::new()
            .pos_xyz(-100.0, 0.0, 0.0)
            .vel_xyz(0.0, 0.0, 0.0)
            .mass(10.0),
    );

    let gravity = Interactions::Gravity::new(6000.0).with_particles(&system.all_particles());
    system.add_interaction(gravity);

    /* // add a non_penetrate constraint to all particles
    let mut index: usize = 0;
    for ref1 in &system.all_particles() {
        for ref2 in &system.all_particles()[(index + 1)..] {
            system.add_constraint(
                Constraints::NonPenetrate::new([*ref1, *ref2], 2.0 * system.particle_radius)
                    .compliance(0.00001)
                    .dissipation(30.0)
                    .as_force(),
            );
        }
        index += 1;
    } */

    system.add_interaction(
        Interactions::LennardJones::new(1000.0, 2. * system.particle_radius)
            .build()
            .with_particles(&system.all_particles()),
    );

    system.static_constraint_pass(1);

    window.run(system);
}

fn print_ang_momentum(_renderer: &mut Particle2DRenderer, system: &mut System) {
    let mut total = Vec3::zero();

    for particle in &mut system.particles {
        total += particle.pos.cross(particle.mass * particle.vel);
    }

    println!("{:#?}", total);
}
