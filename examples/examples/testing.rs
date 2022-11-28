use engine::prelude::*;
use rand::Rng;
use rendering::particle_3d_renderer::Particle3DRenderer;

const COUNT: u32 = 300;
const DENSITY: f64 = 0.005;
const MIN_MASS: f64 = 5.0;
const MAX_MASS: f64 = 100.0;
const GRAVITY: f64 = 600.0;

fn main() {
    let mut system = System::new();
    system.particle_radius =
        ((3.0 * (MIN_MASS + MAX_MASS) / 2.0) / (4.0 * DENSITY * engine::math::PI)).cbrt();
    let mut window = Particle3DRenderer::new();
    window.scale.physics_dt = 1.0 / 30.0;
    window
        .style
        .group_colors
        .insert(1, rendering::colors::FOREST_GREEN);

    let mut rng = rand::thread_rng();
    let bounds = [-500.0, 500.0];

    for _ in 0..COUNT {
        let rand_x = rng.gen_range(bounds[0]..bounds[1]);
        let rand_y = rng.gen_range(bounds[0]..bounds[1]);
        let rand_z = rng.gen_range(bounds[0]..bounds[1]);
        let rand_mass = rng.gen_range(MIN_MASS..MAX_MASS);

        system.add_particle(
            Particle::new()
                .pos_xyz(rand_x, rand_y, rand_z)
                .mass(rand_mass)
                .group(1),
        );
    }

    let mut gravity = Interactions::Falling::new(GRAVITY).build();
    gravity.add_particles(&system.all_particles());
    system.add_interaction(gravity);

    // add a non_penetrate constraint to all particles
    let mut index: usize = 0;
    for ref1 in &system.all_particles() {
        for ref2 in &system.all_particles()[(index + 1)..] {
            system.add_constraint(
                Constraints::NonPenetrate::new([*ref1, *ref2], 2.0 * system.particle_radius)
                    .build(),
            );
        }
        index += 1;
    }

    // add a boundary constraint to all particles
    for part in &system.all_particles() {
        system.add_constraint(
            Constraints::ContactPlane::new(
                *part,
                system.particle_radius,
                Vec3::new(bounds[0], 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
            )
            .build(),
        );
        system.add_constraint(
            Constraints::ContactPlane::new(
                *part,
                system.particle_radius,
                Vec3::new(bounds[1], 0.0, 0.0),
                Vec3::new(-1.0, 0.0, 0.0),
            )
            .build(),
        );
        system.add_constraint(
            Constraints::ContactPlane::new(
                *part,
                system.particle_radius,
                Vec3::new(0.0, bounds[0], 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )
            .build(),
        );
        system.add_constraint(
            Constraints::ContactPlane::new(
                *part,
                system.particle_radius,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, -1.0),
            )
            .build(),
        );
        system.add_constraint(
            Constraints::ContactPlane::new(
                *part,
                system.particle_radius,
                Vec3::new(0.0, 0.0, bounds[0]),
                Vec3::new(0.0, 0.0, 1.0),
            )
            .build(),
        );
    }

    system.static_constraint_pass(2);

    window.run(system);
}


/* use engine::prelude::*;
use rendering::particle_3d_renderer::Particle3DRenderer;

fn main() {
    let mut system = System::new();
    let window = Particle3DRenderer::new();

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

    let mut gravity = Interactions::Gravity::new(40000.0).build();
    gravity.add_particles(&system.all_particles());
    system.add_interaction(gravity);

    window.run(system);
} */
