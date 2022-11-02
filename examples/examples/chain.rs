use engine::prelude::*;
use rendering::particle_2d_renderer::Particle2DRenderer;

const LINK_COUNT: u32 = 75;
const LINK_MASS: f64 = 15.0;
const LINK_RADIUS: f64 = 2.5;
const LINK_GAP: f64 = 1.5;
const GRAVITY: f64 = 200.0;

fn main() {
    let mut system = System::new();
    let mut window = Particle2DRenderer::new();
    window.scale.physics_dt = 1.0 / 120.0;
    window.style.stroke_size = 0.0;

    for i in 0..LINK_COUNT {
        system.add_particle(
            Particle::new()
                .pos_xyz((i as f64) * (2.0 * LINK_RADIUS + LINK_GAP), 0.0, 0.0)
                .radius(LINK_RADIUS)
                .mass(if i == 0 { 0.0 } else { LINK_MASS }),
        );
    }

    for (i, p1) in system.all_particles().iter().enumerate() {
        if (i as u32) != 0 {
            system.add_constraint(Constraints::Distance::new(
                [*p1, *&system.all_particles()[(i as usize) - 1]],
                2.0 * LINK_RADIUS + LINK_GAP,
            ));
        }

        if (i as u32) < LINK_COUNT - 1 {
            system.add_constraint(Constraints::Distance::new(
                [*p1, *&system.all_particles()[(i as usize) + 1]],
                2.0 * LINK_RADIUS + LINK_GAP,
            ));
        }
    }

    system.add_particle(
        Particle::new()
            .pos_xyz(100.0, -250.0, 0.0)
            .radius(20.0)
            .mass(0.0),
    );

    let mut index: usize = 0;
    for ref1 in &system.all_particles() {
        for ref2 in &system.all_particles()[(index + 1)..] {
            system.add_constraint(Constraints::NonPenetrate::new([*ref1, *ref2], false));
        }
        index += 1;
    }

    let mut gravity = Fields::Falling::new(GRAVITY).ground_reference(-500.0);
    gravity.add_particles(&system.all_particles());

    system.add_field(gravity);
    system.static_constraint_pass(0);

    window.run(system);
}
