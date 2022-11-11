use engine::prelude::*;
use rendering::particle_2d_renderer::Particle2DRenderer;

fn main() {
    let mut system = System::new();
    let mut window = Particle2DRenderer::new();
    window.scale.physics_dt = 1.0 / 240.0;

    let bounds = [-500.0, 500.0, -500.0, 500.0];

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

    let mut gravity = Interactions::Gravity::new(6000.0).build();
    gravity.add_particles(&system.all_particles());
    system.add_interaction(gravity);

    // add a non_penetrate constraint to all particles
    let mut index: usize = 0;
    for ref1 in &system.all_particles() {
        for ref2 in &system.all_particles()[(index + 1)..] {
            system.add_constraint(
                Constraints::NonPenetrate::new([*ref1, *ref2], 2.0 * system.particle_radius, false)
                    .compliance(0.00001)
                    .dissipation(30.0),
            );
        }
        index += 1;
    }

    /* let mut repulsion = Fields::VanDerWaals::new(100.0, None, 0.0);
    repulsion.add_particles(&system.all_particles());
    system.add_field(repulsion); */

    // add a boundary constraint to all particles
    for part in &system.all_particles() {
        system.add_constraint(Constraints::ContactPlane::new(
            *part,
            system.particle_radius * 2.0,
            Vec3::new(bounds[0], 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            false,
        ));
        system.add_constraint(Constraints::ContactPlane::new(
            *part,
            system.particle_radius * 2.0,
            Vec3::new(bounds[1], 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            false,
        ));
        system.add_constraint(Constraints::ContactPlane::new(
            *part,
            system.particle_radius * 2.0,
            Vec3::new(0.0, bounds[2], 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            false,
        ));
    }

    system.static_constraint_pass(1);

    window.run(system);
}
