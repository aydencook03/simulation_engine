# Simulation Engine

## Running the Examples
The list of current examples can be found in the examples directory. Before running them, make sure that you have Rust's build tool, Cargo, installed ([installation](https://www.rust-lang.org/tools/install)). You can then run them with 
```console
cargo run --release --example {example_name}
```

## Project Outline

### Introduction

The goal of this project is to create a general purpose framework for particle based simulations. In reality everything is composed of particles, and so it seems reasonable that one should be able to use particle dynamics to simulate (at least to some degree) any kind of phenomenon. Along with this generality of particles, another major advantage of their use is the ability to massively parallelize and distribute the compuational workloads that arise when performing simulations.

The theoretical foundations of this framework are those of classical mechanics:
- The structure of the spacetime is given by the Galilean Group:
    - Euclidean space, a universal time, and symmetry under galilean boosts.
    - The symmetries give rise to the conservation of energy, momentum, angular momentum, and velocity of the center of mass. Using a lagrangian can make it easier to ensure these conservation laws are respected.
- Newton's three laws of motion:
    - $\dot{\vec{v}}\neq\vec{0}\iff\Sigma\vec{F}\neq\vec{0}$
    - $\dot{\left\lbrack\matrix{\vec{r}\cr\vec{p}}\right\rbrack}=\left\lbrack\matrix{\vec{p}/m\cr\Sigma\vec{F}}\right\rbrack$
    - $\vec{F} _{ij}=-\vec{F} _{ji}$
- Newton's second law shows that $m\vec{v}=\vec{p}$ and $\dot{\vec{p}}=\Sigma\vec{F}$, so $\Sigma\vec{F}=m\dot{\vec{v}}+\dot{m}\vec{v}$. The first law says that if there is no external force, then there is no change in velocity, so therefore $\dot{m}=0$. In other words, the mass of a particle must remain constant.
- The laws of thermodynamics must hold.

Due to the small size of particles (relative to us), macroscopic phenomenon appear to virtually operate over a continuum, causing many models to be governed by sets of partial differential equations. A common approach to then performing simulations using these models is to discretize the problem domain using some kind of lattice or mesh, on which partial derivatives can be defined and integrated. However, problems arise when the things being simulated are able to move around or when the mesh/lattice can itself undergo large deformations, in which cases the derivatives are no longer defined and the simulation will break.

This also seemingly leads to a major problem, if most things occur at such a microscopic scale that they seem to be continuous, then trying to simulate them with particles seems hopeless. However, using Newton's second law and summing over a system of particles, it can be shown that an entire system of particles dynamically behave as just a single particle at their center of mass. This allows us to break a system of a computationally impossible number of particles into a more manageable number of particles. The only downside to chunking together groups of particles is that we then lose the emergent phenomenon of continuity/smoothness. Thankfully, there is a mathematically sound way of "smoothing out" the smaller system of larger particles called Smoothed Particle Hydrodynamics ([SPH](https://en.wikipedia.org/wiki/Smoothed-particle_hydrodynamics)).

The other interesting thing to note is that when you use a single particle to represent a collection of particles, you also need to account for two additional degrees of freedom, namely the size and the internal energy (temperature) of that group of particles. This is where different [thermodynamic state equations](https://en.wikipedia.org/wiki/Equation_of_state) and [constitutive models](https://en.wikipedia.org/wiki/Constitutive_equation) will come in. In the limit of many small particles, any **classical** phenomenon will be fully reproducible, and having sph-like smoothing with size and temperature will be a very good approximation at a larger scale.

With all of this in mind, the most fundamental calculations that this engine will be doing is an integration of Newton's second law, the handling of constraints, smoothing of a system of particles to recover a continuum when needed, and evolving some thermodynamic models. To ensure the conservation of energy, a [symplectic integrator](https://en.wikipedia.org/wiki/Symplectic_integrator) will be used. For the handling of constraints, the [XPBD](https://doi.org/10.1145/2994258.2994272) algorithm will be used, which is a fantastic algorithm and framework for compliant constrained dynamics that can even replace things like Finite Element Analysis using particles.

### Primary Goals

- Particles
    - A dynamical object that has mass and a state (position, velocity, temperature, size).
    - Additional properties can be attached to a particle to allow for more dynamics (charge, material properties, etc).
    - Allow kinematic-only particles for things like central gravity bodies, boundary particles, constraint & spring attachments, platforms, etc.
- Fields
    - A generic object that can store state, evolve over time, and dynamically interact with coupled particles.
    - A good api for common functionality of any type of field and boundary condition.
- Compliant Constraints
    - An implementation of the XPBD algorithm.
    - A good api for common functionality of any type of constraint.
- SPH
    - Functions and algorithms that separate away the common functionality of SPH-like smoothing.
- Thermodynamics
    - A good way to handle generic thermodynamic models and equations of state.

### Current State & Challenges

Currently, there are initial implementations of particles, fields, and constraints.

The primary challenge so far has been abstracting away common functionality of fields and constraints so that the engine can universally support a wide variety of dynamics.

Another large challenge has been implementing a good thermodynamics model.

### Project Plan

Initially I want to make sure there is a strong foundation of code that enables all of the primary goals mentioned. A more detailed and task oriented breakdown can be found in the [TODO.md](./TODO.md) file.

## References