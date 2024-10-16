# Simulation Engine

## Examples

|                                                  |                                            |                                            |
|--------------------------------------------------|--------------------------------------------|--------------------------------------------|
| ![Two planets colliding.](./media/collision.gif) | ![Snapping chain.](./media/chain.gif)      | ![Phase transition.](./media/freezing.gif) |
| ![Pile of particles.](./media/pile.gif)          | ![Triple pendulum.](./media/pendulum.gif)  | ![Frictionless block.](./media/block.gif)  |

## Running the Examples
The list of current examples can be found in the examples directory. Before running them, make sure that you have Rust's build tool, Cargo, installed ([installation](https://www.rust-lang.org/tools/install)). You can then run them with 
```console
cargo run --release --example {example_name}
```

## Introduction

### Goal

The goal of this project is to create a general purpose framework for particle based simulations. Because everything in (classical) reality is composed of particles, it seems reasonable that one should be able to use particle dynamics to simulate, or at least approximate, any kind of phenomenon. More precisely, this project aims to make possible the simulation of any non-quantum, non-relativistic phenomenon, through the use of particles.

Along with their generality, another major advantage of the use of particles is the ability to then massively parallelize and distribute the computational workloads that arise when performing simulations.

### Methods

As of now, there are three methods that are planned to form the core of the framework: typical force-based dynamics, extended position-based dynamics (XPBD), and smoothed-particle hydrodynamics (SPH).