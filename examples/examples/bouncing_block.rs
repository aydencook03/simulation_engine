use engine::prelude::*;
use rendering::particle_2d_renderer::Particle2DRenderer;

const GRAVITY: f64 = 600.0;

fn main() {
    let mut system = System::new();
    let mut window = Particle2DRenderer::new();
    window.scale.physics_dt = 1.0 / 240.0;

    let center = Vec3::new(0.0, 0.0, 0.0);
    let vel = Vec3::new(100.0, 70.0, 0.0);
    let angle = engine::math::PI / 5.0;
    let width = 200.0;
    let height = 100.0;

    let top_right = system.add_particle(
        Particle::new()
            .pos(
                engine::math::Matrix3::rotation_axis_angle(Vec3::z_hat(), angle)
                    * Vec3::new(width / 2.0, height / 2.0, 0.0)
                    + center,
            )
            .vel(vel),
    );

    let top_left = system.add_particle(
        Particle::new()
            .pos(
                engine::math::Matrix3::rotation_axis_angle(Vec3::z_hat(), angle)
                    * Vec3::new(-width / 2.0, height / 2.0, 0.0)
                    + center,
            )
            .vel(vel),
    );

    let bottom_left = system.add_particle(
        Particle::new()
            .pos(
                engine::math::Matrix3::rotation_axis_angle(Vec3::z_hat(), angle)
                    * Vec3::new(-width / 2.0, -height / 2.0, 0.0)
                    + center,
            )
            .vel(vel),
    );

    let bottom_right = system.add_particle(
        Particle::new()
            .pos(
                engine::math::Matrix3::rotation_axis_angle(Vec3::z_hat(), angle)
                    * Vec3::new(width / 2.0, -height / 2.0, 0.0)
                    + center,
            )
            .vel(vel),
    );

    system.add_constraint(Constraints::Distance::new([top_right, top_left], width).build());
    system.add_constraint(Constraints::Distance::new([top_left, bottom_left], height).build());
    system.add_constraint(Constraints::Distance::new([bottom_left, bottom_right], width).build());
    system.add_constraint(Constraints::Distance::new([bottom_right, top_right], height).build());
    system.add_constraint(
        Constraints::Distance::new(
            [top_left, bottom_right],
            Vec3::new(width, height, 0.0).mag(),
        )
        .build(),
    );
    system.add_constraint(
        Constraints::Distance::new(
            [bottom_left, top_right],
            Vec3::new(width, height, 0.0).mag(),
        )
        .build(),
    );

    let mut gravity = Interactions::Falling::new(GRAVITY).build();
    gravity.add_particles(&system.all_particles());
    system.add_interaction(gravity);

    // add a boundary constraint to all particles
    for part in &system.all_particles() {
        system.add_constraint(
            Constraints::ContactPlane::new(
                *part,
                system.particle_radius,
                Vec3::new(-500.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
            )
            .as_force()
            .build(),
        );
        system.add_constraint(
            Constraints::ContactPlane::new(
                *part,
                system.particle_radius,
                Vec3::new(500.0, 0.0, 0.0),
                Vec3::new(-1.0, 0.0, 0.0),
            )
            .as_force()
            .build(),
        );
        system.add_constraint(
            Constraints::ContactPlane::new(
                *part,
                system.particle_radius,
                Vec3::new(0.0, -500.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )
            .as_force()
            .build(),
        );
    }

    system.static_constraint_pass(2);

    window.run(system);
}
