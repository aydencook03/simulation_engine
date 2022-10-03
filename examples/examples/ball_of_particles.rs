use engine::prelude::*;
use rand::Rng;
use rendering::particle_2d_renderer::Particle2DRenderer;

const COUNT: u32 = 300;
const DENSITY: f64 = 1.0;
const MIN_MASS: f64 = 5.0;
const MAX_MASS: f64 = 100.0;
const GRAVITY: f64 = 1800.0;

const LARGEST_RADIUS_CUB: f64 = (3.0 * MAX_MASS) / (4.0 * DENSITY);

fn main() {
    let mut system = System::new();
    system.substeps = 50;
    let window = Particle2DRenderer::new();

    let mut rng = rand::thread_rng();
    let back_bottom_left = Vec3::new(-500.0, -500.0, -500.0);
    let front_top_right = Vec3::new(500.0, 500.0, 500.0);

    for _ in 0..COUNT {
        let rand_x = rng.gen_range(
            (back_bottom_left.x + LARGEST_RADIUS_CUB.cbrt())
                ..(front_top_right.x - LARGEST_RADIUS_CUB.cbrt()),
        );
        let rand_y = rng.gen_range(
            (back_bottom_left.y + LARGEST_RADIUS_CUB.cbrt())
                ..(front_top_right.y - LARGEST_RADIUS_CUB.cbrt()),
        );
        let rand_mass = rng.gen_range(MIN_MASS..MAX_MASS);

        system.add_particle(
            Particle::new()
                .pos_xyz(rand_x, rand_y, 0.0)
                .mass(rand_mass)
                .radius_from_density(DENSITY),
        );
    }

    let mut gravity = NGravity::new(GRAVITY * 10.0, 0.0);
    gravity.add_particles(&system.all_particles());
    system.add_field(gravity);

    let mut no_overlap = NoOverlapConstraint::new();
    no_overlap.add_particles(&system.all_particles());
    system.add_field(no_overlap);

    window.run(system);
}
