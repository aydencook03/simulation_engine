use engine::prelude::*;
use rendering::particle_2d_renderer::Particle2DRenderer;

fn main() {
    let mut system = System::new();
    let window = Particle2DRenderer::new();

    let back_bottom_left = Vec3::new(-500.0, -500.0, -500.0);
    let front_top_right = Vec3::new(500.0, 500.0, 500.0);

    system.add_particle(
        Particle::new()
            .pos_xyz(-50.0, 0.0, 0.0)
            .vel_xyz(50.0, 0.0, 0.0)
            .radius(20.0)
            .mass(40.0),
    );

    system.add_particle(
        Particle::new()
            .pos_xyz(50.0, 0.0, 0.0)
            .vel_xyz(-50.0, 0.0, 0.0)
            .radius(2.0)
            .mass(1.0),
    );

    let mut no_overlap = NoOverlapConstraint::new();
    no_overlap.add_particles(&system.all_particles());
    system.add_field(no_overlap);

    let mut rect_bound = RectBoundConstraint::new(back_bottom_left, front_top_right);
    rect_bound.add_particles(&system.all_particles());
    system.add_field(rect_bound);

    window.run(system);
}
