use engine::prelude::*;
use rendering::particle_2d_renderer::Particle2DRenderer;

fn main() {
    let mut system = System::new();
    let window = Particle2DRenderer::new();

    let bounds = [-500.0, 500.0, -500.0, 500.0];

    system.add_particle(
        Particle::new()
            .pos_xyz(100.0, 0.0, 0.0)
            .vel_xyz(0.0, 10.0, 0.0)
            .radius(5.0)
            .mass(0.4),
    );
    system.add_particle(
        Particle::new()
            .pos_xyz(-100.0, 0.0, 0.0)
            .vel_xyz(0.0, 0.0, 0.0)
            .radius(20.0),
    );

    let mut gravity = Fields::Gravity::new(60000.0);
    gravity.add_particles(&system.all_particles());
    system.add_field(gravity);

    // add a non_penetrate constraint to all particles
    let mut index: usize = 0;
    for ref1 in &system.all_particles() {
        for ref2 in &system.all_particles()[(index + 1)..] {
            system.add_constraint(
                Constraints::NonPenetrate::new([*ref1, *ref2], false)
                    .compliance(0.0001)
                    .dissipation(10.0),
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
            Vec3::new(bounds[0], 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            false,
        ));
        system.add_constraint(Constraints::ContactPlane::new(
            *part,
            Vec3::new(bounds[1], 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            false,
        ));
        system.add_constraint(Constraints::ContactPlane::new(
            *part,
            Vec3::new(0.0, bounds[2], 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            false,
        ));
    }

    system.static_constraint_pass(1);

    window.run(system);
}
