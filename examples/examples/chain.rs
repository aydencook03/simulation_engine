use engine::prelude::*;
use rendering::particle_2d_renderer::Particle2DRenderer;

const CHAIN_LENGTH: f64 = 400.0;
const CHAIN_MASS: f64 = 0.935;
const LINK_COUNT: u32 = 200;
const GRAVITY: f64 = 275.0;

const LINK_RADIUS: f64 = (CHAIN_LENGTH / (LINK_COUNT as f64)) / 2.0;
const LINK_MASS: f64 = CHAIN_MASS / (LINK_COUNT as f64);

fn main() {
    let mut system = System::new();
    system.particle_radius = LINK_RADIUS;
    let mut window = Particle2DRenderer::new();
    window.scale.physics_dt = 1.0 / 60.0;
    window.style.stroke_size = 0.0;

    for i in 0..LINK_COUNT {
        system.add_particle(
            Particle::new()
                .pos_xyz((i as f64) * (2.0 * LINK_RADIUS), 0.0, 0.0)
                .mass(if i == 0 { 0.0 } else { LINK_MASS }),
        );
    }

    for (i, p1) in system.all_particles().iter().enumerate() {
        if (i as u32) < LINK_COUNT - 1 {
            system.add_constraint(
                Constraints::Distance::new(
                    [*p1, *&system.all_particles()[(i as usize) + 1]],
                    2.0 * LINK_RADIUS,
                )
                .max_tension(226000.0 / ((i + 1) as f64)),
            );
        }
    }

    system.add_particle(Particle::new().pos_xyz(100.0, -250.0, 0.0).mass(0.0));
    system.add_particle(Particle::new().pos_xyz(60.0, -250.0, 0.0).mass(0.0));

    let mut index: usize = 0;
    for ref1 in &system.all_particles() {
        for ref2 in &system.all_particles()[(index + 1)..] {
            system.add_constraint(Constraints::NonPenetrate::new(
                [*ref1, *ref2],
                2.0 * system.particle_radius,
                true,
            ));
        }
        index += 1;
    }

    let mut gravity = Fields::Falling::new(GRAVITY).ground_reference(-500.0);
    gravity.add_particles(&system.all_particles());

    system.add_field(gravity);
    system.static_constraint_pass(1);

    window.run(system);
}
