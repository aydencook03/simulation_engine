# Rusty Physics Engine

## How to build

## Capabilities

## Why particle based?

## Technical Scope

In order to help fight the urge of wanting to add more 
and more complex features, I've outlined the scope of 
this engine.

This is a classical, non-relativistic, non-field-theoretic, particle based physics engine.

But what does this mean exactly?
- The most fundamental object is a particle.
- These particles behave dynamically according to $$\frac{d}{dt}\begin{bmatrix}\vec{r} \\\ \vec{p}\end{bmatrix}=\begin{bmatrix}\vec{p}/m \\\ \Sigma\vec{F}\end{bmatrix}$$
- There is no massless particle (these are only permitted in a relativistic theory).
- Particles do not possess angular momentum (no intrinsic spin).
- Mass isn't a measure of rest energy, so it is conserved.
- Interactions are non-local, as there are no fields to mediate forces.

This boils it down to only three conservation laws:
- Linear momentum
- Energy
- Mass

## References, Ideas, Inspiration:

- Youtube [playlist](https://youtube.com/playlist?list=PLvypLlLlZuNhcdtPKfQ25cpmhBuWWDZzR) for ideas.
- Work of [Matthias Müller](https://matthias-research.github.io/pages/) and [Miles Macklin](http://blog.mmacklin.com/).
- Papers:
    - Müller, Heidelberger, Hennix, Ratcliff. 2007. [Link](https://doi.org/10.1016/j.jvcir.2007.01.005).<br />Position based dynamics.
    - Macklin, Müller. 2013. [Link](https://doi.org/10.1145/2461912.2461984). <br />Position based fluids.
    - Macklin, Müller, Chentanez, Kim. 2014. [Link](https://doi.org/10.1145/2601097.2601152).<br />Unified particle physics for real-time applications.
    - Takahashi, Nishita, Fujishiro. 2013. [Link](https://doi.org/10.1016/j.cag.2014.06.002).<br />Fast simulation of viscous fluids with elasticity and thermal conductivity using position-based dynamics.