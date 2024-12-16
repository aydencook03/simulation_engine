# Particle-Based Simulation Engine

A powerful, general-purpose framework for particle-based simulations in Rust.

This Rust-based engine provides a flexible framework for particle-based simulations, supporting methods like Force-Based Dynamics (FBD) and Extended Position-Based Dynamics (XPBD). XPBD enables both rigid and soft body simulations, along with arbitrary constraints like collisions, joints, and springs.

Examples include simulations of colliding bodies, a breaking chain under stress, a frictionless rigid block, Lennard-Jones gases, and phase transitions. A basic real-time 2D renderer is included as an example implementation to visualize these systems, built using low-level Rust libraries like `winit` and `softbuffer` for efficient and lightweight rendering.

Smoothed Particle Hydrodynamics (SPH) is a planned feature that will enable fluid and other continuous material simulations, such as modeling liquid flow or astrophysical phenomena.

The engine is modular, allowing users to create custom interactions and constraints to suit their needs.

## Key Features

- Multiple simulation methods: Force-based Dynamics (FBD), Extended Position-Based Dynamics (XPBD), and Smoothed-Particle Hydrodynamics (SPH) (Planned)
- Rigid body simulation using both XPBD and FBD
- Fluid simulation using SPH (Planned)
- Modular architecture for easy extension
- Real-time 2D rendering
- Pre-implemented interactions and constraints
- Support for custom user-defined interactions and constraints

## Examples

|                                                  |                                            |                                            |
|--------------------------------------------------|--------------------------------------------|--------------------------------------------|
| ![Two planets colliding.](./media/collision.gif) | ![Snapping chain.](./media/chain.gif)      | ![Phase transition.](./media/freezing.gif) |
| ![Pile of particles.](./media/pile.gif)          | ![Triple pendulum.](./media/pendulum.gif)  | ![Frictionless block.](./media/block.gif)  |

More examples can be found in the `examples/examples` directory.

## Quick Start

1. Install Rust and Cargo: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Clone the repository:

   ```bash
   git clone https://github.com/aydencook03/simulation_engine.git
   cd simulation_engine
   ```

3. Run an example:

   ```bash
   cargo run --release --example collision
   ```

To run other examples, replace `collision` with the name of any example file found in the `examples/examples` directory.

## Core Methods

1. **Force-based Dynamics (FBD)**: Applies Newtonian mechanics directly to particles, suitable for rigid body dynamics, gravitational systems, and force fields.

2. **Extended Position-Based Dynamics (XPBD)**: A constraint-based method that directly manipulates particle positions, effective for complex interactions like collisions and joint constraints.

3. **Smoothed-Particle Hydrodynamics (SPH)**: A mesh-free approach for simulating fluids and continuous materials, versatile for fluid flow, astrophysics, soft matter, and thermodynamic simulations.

## Project Structure

- `engine`: Core simulation engine
- `rendering`: Visualization utilities
- `examples`: Example simulations

## Core Concepts

- **System**: Central component managing particles, interactions, and constraints
- **Particles**: Fundamental units with properties like position, velocity, and mass
- **Interactions**: Define inter-particle interactions (e.g., generic forces, gravity, Lennard-Jones potential)
- **Constraints**: Enforce conditions on particles (e.g., distance constraints, collisions)

## Simulation Loop

The engine uses a distinct simulation loop that separates interactions and constraints:

1. Apply interactions (forces) to particles
2. Integrate particle positions
3. Apply constraints to enforce specific conditions
4. Update particle velocities

This separation allows for efficient handling of both dynamic force-based behaviors and precise geometric constraints.

## Extending the Engine

1. Implement the `Interaction` trait for new interactions
2. Implement the `Constraint` trait for new constraints
3. Use `XpbdParameters` to integrate new constraints with the XPBD solver

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Dual-licensed under the MIT License and the Apache License 2.0.
