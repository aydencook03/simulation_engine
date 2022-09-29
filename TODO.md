# TODO / IDEAS

- [ ] generate README from lib docs

- dynamic interaction
  - [ ] pause / play
  - [ ] step forward
  - [ ] reset
  - [ ] serialization of System

- possibly useful crates
  - [ ] bevy_ecs
  - [ ] bevy (for rendering and interaction)
  - [ ] nalgebra
  - [ ] parry
  - [ ] rayon / bevy_tasks
  - [ ] serde

- rendering
  - [ ] MVP real time renderer (w/ GUI & interaction?)
  - [ ] baked animation renderer

- physics
  - [ ] realistic forces
    - [ ] newtonian gravity
    - [ ] van der waals
    - [ ] chemical bonding like forces
    - [ ] linear/spring force
    - [ ] collision / impulse forces
    - [ ] fluid forces (attraction & repulsion)
  - [ ] fields
    - [ ] electromagnetic
    - [ ] material point method mesh
  - [ ] friction
  - [ ] viscosity
  - [ ] soft bodies
    - [ ] volume conserving, pnuematic, etc
  - [ ] deformable solids (break constraint and then create a new one in the new location?)
  - [ ] bouncy objects
  - [ ] granular materials
  - [ ] temperature simulation
    - [ ] contact conduction
    - [ ] melting (weaken constraints as temperature rises)
    - [ ] collision heating based on energy lost after constraint solve
    - [ ] color change/glow based on temperature
    - [ ] radiation (temp drops naturally over time)

- long term
  - [ ] good 3d rendering
    - [ ] isosurface/isoline rendering?
  - [ ] load 3d object files as set of connected particles
  - [ ] parallelize using rayon or bevy_tasks::ParallelIterator
  - [ ] wgpu acceleration