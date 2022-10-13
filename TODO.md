# TODO / IDEAS

- Short Term
  - [ ] develop a good abstraction for constraints
  - [ ] simplify the abstraction for fields (ie: remove dt, etc)
  - [ ] linear spring force
  - [ ] real-time 3d renderer w/ simple interaction (using bevy)
  - [ ] support particle sources and sinks
  - [ ] non-dynamic particles (central gravity bodies, boundary particles, constraint & spring attachments, platforms, etc.)
  - [ ] first class sph for smoothing?
  - [ ] investigate/solve weird lack of angular momentum conservation
    - only occurs when multiple fields are being used simultaneously?
  - [ ] builder methods for builtin fields & constraints
  - [ ] fix BoxBound instability

- Fields
  - [ ] particle-mesh-method newtonian gravity
  - [ ] electromagnetic
  - [ ] material point method mesh

- Collision Handling
  - [ ] collision / impulse forces
  - [ ] friction
  - [ ] viscosity

- Thermodynamics
  - [ ] research how to implement a good general macroscopic thermodynamics model
  - [ ] conduction, advection, radiation, collisions
  - [ ] state variables & state equations
    - what is needed in this engine? pos, vel, temp, radius?
    - fields can alter any of these state variables. a field could implement a certain state eqn?
  - [ ] thermal expansion
  - [ ] phase changes

- Compilation Features
  - [ ] serialization of System struct and saving/loading from file (using serde)
  - [ ] object mesh file loading (for FEM-like analysis)?
  - [ ] parallelize using simd, rayon, bevy_tasks::ParallelIterator, etc
  - [ ] wgpu acceleration

- Rendering
  - [ ] animation baking (both 3d and 2d)
    - [ ] save/serialize System state to file once the animation is completed
  - [ ] color based on radiation/temperature

- Algorithms
  - [ ] spawning composite shapes of particles (spheres, blocks, etc)
  - [ ] hash map collision detection
  - [ ] tree methods for optimized n-body forces

- Physics
  - [ ] intermolecular forces
    - [ ] solids (bonds)
      - [ ] xpbd distance constraints w/ compliance
      - [ ] smoothed-particle-hydrodynamics
      - [ ] material-point-method
    - [ ] liquids & gases (intermolecular forces & hydrogen bonds)
      - [ ] Lennard-Jones 6-12, 6-9, 10-12
      - [ ] smoothed-particle-hydrodynamics
      - [ ] material-point-method
      - [ ] position-based-fluids
  - [ ] soft bodies
    - [ ] volume conserving, pnuematic, etc
  - [ ] deformable solids (break constraint and then create a new one in the new location?)
  - [ ] bouncy objects
  - [ ] granular materials

- Long Term
  - [ ] good 3d rendering
    - [ ] isosurface/isoline rendering?
  - [ ] extend engine to support rigid bodies (like bullet, rapier, xpbd, etc)