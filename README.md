# Simulation Engine

## Running the Examples
The list of current examples can be found in the examples directory. Before running them, make sure that you have Rust's build tool, Cargo, installed (more info [here](https://www.rust-lang.org/tools/install)). You can then run them with 
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

This seemingly leads to a major problem

symplectic euler integration of newton's equations, xpbd algorithm (Compliant Constrained Dynamics, constraints, fem, etc), sph

### Primary Goals

### Current State & Challenges

### Project Plan

## References