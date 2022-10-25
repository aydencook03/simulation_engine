use engine::prelude::*;
use rand::Rng;
use rendering::particle_2d_renderer::Particle2DRenderer;

const STAR_PARTICLE_COUNT: u32 = 750;
const STAR_MASS: f64 = 1.989e30;
const STAR_DENSITY: f64 = 1410.0;
const G: f64 = 6.674e-11;

fn main() {
    // configure the system and rendering parameters
    let mut system = System::new();
    let mut window = Particle2DRenderer::new();
    system.substeps = 1;
    window.style.stroke_size = 0.0;
    window.style.bg_color = rendering::colors::BLACK;
    window
        .style
        .group_colors
        .insert(1, rendering::colors::EARTH_BLUE);
    window
        .style
        .group_colors
        .insert(2, rendering::colors::FOREST_GREEN);
    window.style.group_colors.insert(3, rendering::colors::RUST);
    window.scale.starting_zoom = -15.0;
    window.scale.physics_dt = 1.0 * 60.0;
    window.scale.time_unit = (60.0, "Minutes".to_string());

    let star_radius = ((3.0 * STAR_MASS) / (4.0 * 3.1415 * STAR_DENSITY)).cbrt();
    let particle_mass = STAR_MASS / (STAR_PARTICLE_COUNT as f64);
    let mut rng = rand::thread_rng();

    let star_1_pos = Vec3::new(-2.0 * star_radius, 2.0 * star_radius, -2.0 * star_radius);
    let star_1_vel = Vec3::new(
        0.0005 * star_radius,
        -0.0003 * star_radius,
        0.0004 * star_radius,
    );

    let star_2_pos = Vec3::new(2.0 * star_radius, -2.0 * star_radius, 2.0 * star_radius);
    let star_2_vel = Vec3::new(
        -0.0005 * star_radius,
        0.0003 * star_radius,
        -0.00065 * star_radius,
    );

    // create stars
    for _ in 0..STAR_PARTICLE_COUNT {
        let rand = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );

        // star 1
        system.add_particle(
            Particle::new()
                .pos(star_1_pos + rand)
                .vel(star_1_vel)
                .mass(particle_mass)
                .radius_from_density(STAR_DENSITY)
                .group(3),
        );

        // star 2
        system.add_particle(
            Particle::new()
                .pos(star_2_pos + rand)
                .vel(star_2_vel)
                .mass(particle_mass)
                .radius_from_density(STAR_DENSITY)
                .group(rng.gen_range(1..3)),
        );
    }

    // create gravity
    let mut gravity = Fields::NGravity::new(G, 0.0);
    gravity.add_particles(&system.all_particles());
    system.add_field(gravity);

    // add a non_penetrate constraint to all particles
    let mut index: usize = 0;
    for ref1 in &system.all_particles() {
        for ref2 in &system.all_particles()[(index + 1)..] {
            system.add_constraint(Constraints::NonPenetrate::new([*ref1, *ref2]));
        }
        index += 1;
    }

    system.static_constraint_pass(50);

    window.run(system);
}
