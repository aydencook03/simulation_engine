use engine::prelude::*;
use rand::Rng;
use rendering::particle_2d_renderer::Particle2DRenderer;

const COUNT: u32 = 1500;
const DENSITY: f64 = 1410.0;
const MASS: f64 = 1.989e30;
//const RADIUS: f64 = 696000.0;
const G: f64 = 6.674e-11;

fn main() {
    let mut system = System::new();
    system.substeps = 1;
    let mut window = Particle2DRenderer::new();
    window.style.stroke_size = 0.0;
    window.scale.starting_zoom = -14.0;
    window.scale.physics_dt = 1.0 * 60.0;
    window.scale.time_unit = (60.0, "Minutes".to_string());

    let mut rng = rand::thread_rng();

    for _ in 0..COUNT {
        let rand_x = rng.gen_range(-500.0..500.0);
        let rand_y = rng.gen_range(-500.0..500.0);
        let rand_z = rng.gen_range(-500.0..500.0);
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

    system.static_constraint_pass(100);

    window.run(system);
}
