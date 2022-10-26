use engine::prelude::*;
use rand::Rng;
use rendering::particle_2d_renderer::Particle2DRenderer;

const COUNT: u32 = 500;
const DENSITY: f64 = 0.005;
const MIN_MASS: f64 = 5.0;
const MAX_MASS: f64 = 100.0;
const GRAVITY: f64 = 600.0;

fn main() {
    let mut system = System::new();
    let mut window = Particle2DRenderer::new();
    window.scale.physics_dt = 1.0 / 30.0;
    window
        .style
        .group_colors
        .insert(1, rendering::colors::FOREST_GREEN);

    let mut rng = rand::thread_rng();
    let bounds = [-500.0, 500.0, -500.0, 500.0];

    for _ in 0..COUNT {
        let rand_x = rng.gen_range(bounds[0]..bounds[1]);
        let rand_y = rng.gen_range(bounds[2]..bounds[3]);
        let rand_mass = rng.gen_range(MIN_MASS..MAX_MASS);

        system.add_particle(
            Particle::new()
                .pos_xyz(rand_x, rand_y, 0.0)
                .mass(rand_mass)
                .radius_from_density(DENSITY)
                .group(1),
        );
    }

    let mut gravity = Fields::Falling::new(GRAVITY);
    gravity.add_particles(&system.all_particles());
    system.add_field(gravity);

    // add a non_penetrate constraint to all particles
    let mut index: usize = 0;
    for ref1 in &system.all_particles() {
        for ref2 in &system.all_particles()[(index + 1)..] {
            system.add_constraint(Constraints::NonPenetrate::new([*ref1, *ref2], true));
        }
        index += 1;
    }

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
