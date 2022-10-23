use engine::prelude::*;
use rand::Rng;
use rendering::particle_2d_renderer::Particle2DRenderer;

const COUNT: u32 = 1000;
const DENSITY: f64 = 1410.0;
const MASS: f64 = 1.989e30;
const RADIUS: f64 = 696000.0;
const G: f64 = 6.674e-11;

fn main() {
    let mut system = System::new();
    system.running = false;
    system.substeps = 50;
    let mut window = Particle2DRenderer::new();
    window.style.stroke_size = 0.0;
    window.scale.starting_zoom = -14.0;
    window.scale.physics_dt = 1.0 * 60.0 * 60.0;
    window.scale.time_scale = (60.0 * 60.0, "Hours".to_string());

    let mut rng = rand::thread_rng();
    let back_bottom_left = Vec3::new(-500.0, -500.0, -500.0);
    let front_top_right = Vec3::new(500.0, 500.0, 500.0);

    for _ in 0..COUNT {
        let rand_x = rng.gen_range(back_bottom_left.x..front_top_right.x);
        let rand_y = rng.gen_range(back_bottom_left.y..front_top_right.y);
        let rand_z = rng.gen_range(back_bottom_left.z..front_top_right.z);
        let mass = MASS / (COUNT as f64);

        system.add_particle(
            Particle::new()
                .pos_xyz(rand_x, rand_y, rand_z)
                .mass(mass)
                .radius_from_density(DENSITY),
        );
    }

    let mut gravity = Fields::NGravity::new(G, 0.0);
    gravity.add_particles(&system.all_particles());
    system.add_field(gravity);

    let mut index: usize = 0;
    for ref1 in &system.all_particles() {
        for ref2 in &system.all_particles()[(index + 1)..] {
            system.add_constraint(Constraints::NonPenetrate::new([*ref1, *ref2]));
        }
        index += 1;
    }

    system.static_constraint_pass(1000);

    window.run(system);
}
