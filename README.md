# Particle-Based Simulation Engine

## Examples

|                                                  |                                            |                                            |
|--------------------------------------------------|--------------------------------------------|--------------------------------------------|
| ![Two planets colliding.](./media/collision.gif) | ![Snapping chain.](./media/chain.gif)      | ![Phase transition.](./media/freezing.gif) |
| ![Pile of particles.](./media/pile.gif)          | ![Triple pendulum.](./media/pendulum.gif)  | ![Frictionless block.](./media/block.gif)  |

## Running the Examples

The list of current examples can be found in the `examples` directory. Before running them, ensure that you have Rust's build tool, Cargo, installed ([installation guide](https://www.rust-lang.org/tools/install)). Once installed, you can run any example using:

```bash
cargo run --release --example {example_name}
```

Replace `{example_name}` with the name of the example you want to run (e.g., `collision`, `pendulum`, etc.).

## Introduction

### Goal

The purpose of this project is to develop a general-purpose framework for particle-based simulations. Since everything in classical reality can be modeled as particles, this framework is designed to simulate—or at least approximate—various phenomena, from physical interactions to emergent behaviors. It focuses on non-quantum, non-relativistic phenomena, which are most effectively modeled through particle dynamics.

One of the core advantages of a particle-based approach is its potential for massive parallelization and distribution, making large-scale simulations feasible across different hardware architectures.

### Methods

The project is currently built around three core simulation methods:

1. **Force-based Dynamics (FBD)**\
   This method applies Newtonian mechanics directly to particles, using forces and accelerations to update their positions and velocities. It's suitable for simulating rigid body dynamics, gravitational systems, force fields, and similar phenomena where forces are well-defined. This general-purpose approach also allows for simulations of mesoscopic systems, dissipative particle dynamics, and the material point method.

2. **Extended Position-Based Dynamics (XPBD)**\
   XPBD is a constraint-based method that focuses on directly manipulating the positions of particles to enforce constraints, making it effective for handling complex interactions like collisions, joint constraints, and deformable bodies. It is commonly used in soft-body simulations, cloth physics, and articulated structures.

3. **Smoothed-Particle Hydrodynamics (SPH)**\
   SPH is a mesh-free particle-based approach for simulating fluids and other continuous materials. It uses particles to represent fluid elements, making it highly flexible for fluid flow, soft matter, solid deformation, astrophysical simulations, and thermodynamic simulations. SPH's ability to handle continuous media without requiring an underlying grid makes it versatile for simulating liquids, gases, and even granular materials.

## Key Features

- Multiple simulation methods: FBD, XPBD, and SPH
- Modular architecture allowing easy extension of particle behaviors
- Included real-time 2D rendering for visualization
- Various pre-implemented interactions and constraints
- Support for custom user-defined interactions and constraints

## Getting Started

### Prerequisites

- Rust and Cargo ([installation guide](https://www.rust-lang.org/tools/install))

### Running Examples

1. Clone the repository:
   ```
   git clone https://github.com/aydencook03/simulation_engine.git
   cd simulation_engine
   ```

2. Run an example:
   ```
   cargo run --release --example {example_name}
   ```
   Replace `{example_name}` with the name of the example you want to run (e.g., `collision`, `chain`, `freezing`, etc.).

## Project Structure

The project is organized into several Rust crates:

- `engine`: Core simulation engine
- `rendering`: Rendering utilities for visualization
  - `particle_2d_renderer`: 2D rendering implementation
  - `colors`: Color definitions
  - `view_2d`: 2D view management
- `examples`: Example simulations demonstrating the engine's capabilities

## Key Components

### System

The `System` struct is the central component of the simulation. It manages particles, interactions, and constraints, and handles the time evolution of the simulation.

### Particles

Particles are the fundamental units of the simulation. Each particle has properties such as position, velocity, mass, and can belong to a group.

### Interactions

Interactions define how particles influence each other through forces. They are applied at the beginning of each simulation step, before particle integration. The engine provides several pre-implemented interactions, such as:

- Gravity
- Lennard-Jones potential
- Custom force fields

Interactions typically model physical forces between particles or external forces acting on particles. They modify particle forces, which then affect velocities and positions during integration.

### Constraints

Constraints enforce specific conditions on particles or groups of particles. They are applied after particle integration in each simulation step. The engine uses XPBD (Extended Position-Based Dynamics) for constraint solving, allowing for both equality and inequality constraints. Examples include:

- Distance constraints
- Non-penetration constraints
- Contact plane collisions

Constraints directly modify particle positions (and sometimes velocities) to satisfy certain conditions, maintaining specific relationships between particles or enforcing collision avoidance.

### Key Differences

- **Timing**: Interactions occur before integration, while constraints are applied after.
- **Purpose**: Interactions model forces, while constraints enforce specific conditions.
- **Implementation**: Interactions are applied directly to particle forces, while constraints are applied to particle positions.

This separation allows the engine to handle both dynamic force-based behaviors and precise geometric constraints efficiently.

### Rendering

The project uses a combination of libraries to provide real-time 2D visualization of the simulations:

- **winit**: For creating and managing the window and handling events.
- **softbuffer**: For accessing the window's framebuffer directly.
- **tiny-skia**: For rasterization and drawing shapes.

These libraries are combined in the `Particle2DRenderer` struct to create a 2D rendering system for the particle simulations. The renderer provides a top-down view of the simulation.

### Rendering Controls

The rendering system provides several controls for interacting with the simulation view:

|  Key   |   Action     |
|--------|--------------|
| Arrows | Pan Around   |
| +/-    | Zoom In/Out  |
| Enter  | Reset View   |
| Space  | Pause/Play   |
| F      | Step Forward |
| S      | Save Image   |
| Q      | Quit         |

## Extending the Engine

To create custom interactions or constraints:

1. Implement the `Interaction` trait for new interactions.
2. Implement the `Constraint` trait for new constraints.
3. Use the `XpbdParameters` struct to integrate new constraints with the XPBD solver.

## Future Development

Refer to the TODO.md file for a comprehensive list of planned features and improvements.

## License

This project is dual-licensed under the MIT License and the Apache License 2.0. See the LICENSE-MIT and LICENSE-APACHE files for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgements

This project draws inspiration from various particle-based simulation techniques and libraries in the field of computational physics and computer graphics.
