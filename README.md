# Simulation Engine

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
   SPH is a mesh-free particle-based approach for simulating fluids and other continuous materials. It uses particles to represent fluid elements, making it highly flexible for fluid flow, soft matter, solid deformation, astrophysical simulations, and thermodynamic simulations. SPH’s ability to handle continuous media without requiring an underlying grid makes it versatile for simulating liquids, gases, and even granular materials.