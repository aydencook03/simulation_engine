use engine::prelude::*;
use rand::Rng;
use rendering::particle_2d_renderer::Particle2DRenderer;

const COUNT: u32 = 500;
const MASS: f64 = 10.0;
const RADIUS: f64 = 8.0;
const BOND_ENERGY: f64 = 80000.0;

fn main() {
    let mut system = System::new();
    let mut window = Particle2DRenderer::new();
    window.scale.physics_dt = 1.0 / 30.0;

    let mut rng = rand::thread_rng();
    let bounds = [-500.0, 500.0, -500.0, 500.0];

    for _ in 0..COUNT {
        let rand_x = rng.gen_range((bounds[0] + RADIUS)..(bounds[1] - RADIUS));
        let rand_y = rng.gen_range((bounds[2] + RADIUS)..(bounds[3] - RADIUS));

        system.add_particle(
            Particle::new()
                .pos_xyz(rand_x, rand_y, 0.0)
                .mass(MASS)
                .radius(RADIUS),
        );
    }

    let mut repulsion = Fields::VanDerWaals::new(BOND_ENERGY, None, 0.5);
    repulsion.add_particles(&system.all_particles());
    system.add_field(repulsion);

    for particle in &system.all_particles() {
        system.add_constraint(Constraints::ContactPlane::new(
            *particle,
            Vec3::new(bounds[0], 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            false,
        ));
        system.add_constraint(Constraints::ContactPlane::new(
            *particle,
            Vec3::new(bounds[1], 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            false,
        ));
        system.add_constraint(Constraints::ContactPlane::new(
            *particle,
            Vec3::new(0.0, bounds[2], 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            false,
        ));
        system.add_constraint(Constraints::ContactPlane::new(
            *particle,
            Vec3::new(0.0, bounds[3], 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            false,
        ));
    }

    let mut index: usize = 0;
    let mut constraints: Vec<Constraints::NonPenetrate> = Vec::new();
    for ref1 in &system.all_particles() {
        for ref2 in &system.all_particles()[(index + 1)..] {
            constraints.push(Constraints::NonPenetrate::new([*ref1, *ref2], true));
        }
        index += 1;
    }
    for _ in 0..100 {
        for constraint in &mut constraints {
            constraint.project(&mut system.particles, core::f64::MIN_POSITIVE, true);
        }
        system.static_constraint_pass(1);
    }

    window.run(system);
}
