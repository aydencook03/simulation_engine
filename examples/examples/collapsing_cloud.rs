use engine::prelude::*;
use rand::Rng;
use rendering::particle_2d_renderer::Particle2DRenderer;

const COUNT: u32 = 200;
const MASS: f64 = 10.0;
const RADIUS: f64 = 8.0;
const GRAVITY: f64 = 48000.0;
const BOND_ENERGY: f64 = 8000.0;

fn main() {
    let mut system = System::new();
    let window = Particle2DRenderer::new();

    let mut rng = rand::thread_rng();
    let back_bottom_left = Vec3::new(-500.0, -500.0, -500.0);
    let front_top_right = Vec3::new(500.0, 500.0, 500.0);

    for _ in 0..COUNT {
        let rand_x = rng.gen_range(back_bottom_left.x..front_top_right.x);
        let rand_y = rng.gen_range(back_bottom_left.y..front_top_right.y);

        system.add_particle(
            Particle::new()
                .pos_xyz(rand_x, rand_y, 0.0)
                .mass(MASS)
                .radius(RADIUS),
        );
    }

    let mut gravity = NGravity::new(GRAVITY, Some(0.03));
    gravity.add_particles(&system.all_particles());
    system.add_field(gravity);

    let mut repulsion = VanDerWaals::new(BOND_ENERGY, None, Some(0.03));
    repulsion.add_particles(&system.all_particles());
    system.add_field(repulsion);

    let mut rect_bound = RectBoundConstraint::new(back_bottom_left, front_top_right);
    rect_bound.add_particles(&system.all_particles());
    //system.add_field(rect_bound);

    window.run(system);
}
