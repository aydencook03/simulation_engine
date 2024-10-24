# TODO / IDEAS

- Short Term
  - [ ] 3d renderer
  - [ ] spatial hashing vs bounding boxes vs signed-distance-field-voxelization (speed, generality, ease of implementation)
  - [ ] AABB/BVH pass then SDF voxelization pass
  - [ ] look at structure of Rapier project (parry, Colliders, user api, etc)
  - [ ] sph prototypes
  - [ ] interactivity (interaction in physics pipeline, etc)
  - [ ] investigate
    - [ ] read xpbd paper. understand mathematical derivation to see where my algorithm could come from
    - [ ] cantilever beam w/ xpbd vs w/out
    - [ ] derive constraint function for other potentials (like gravity?) (learn how to from neo-hookean paper)
    - [ ] why does non-xpbd conserve angular momentum while xpbd does not?
    - [ ] why does non-xpbd conserve kinetic energy while xpbd does not?
    - [ ] step through non-xpbd algorithm w/ variables to prove the conservation of quantities
    - [ ] what is the physical correspondance to the xpbd dissipation? what is its behavior in relation to compliance?

- Big Picture
  - math
    - [X] pi
    - [X] point
    - [X] vector
    - [X] matrix
  - objects:
    - [X] system
    - [X] particles
  - physics:
    - [X] interactions
      - [X] generic
      - [X] field interaction
      - [X] pair-wise interaction forces
      - [X] simple force
    - [X] xpbd compliant constraints
    - [ ] sph
    - [ ] collisions (verlet lists)
    - [ ] thermodynamics
    - [ ] molecular dynamics, statistical mechanics, mesoscale modeling, active matter
      - [ ] molecular dynamics => dissipative particle dynamics => smoothed particle hydrodynamics
    - [ ] boundary conditions
      - [ ] sources & sinks
      - [ ] outlet / sink
      - [ ] no-slip
      - [ ] constant pressure
      - [ ] symmetric
      - [ ] periodic
      - [ ] rigid / fixed
  - algorithms:
    - [ ] n-body pairing (take a generic slice and return an iterator of pairs)
    - [ ] composite shapes (rigid body <-> collection of particles) (voxelization)
    - [ ] mesh <-> connected particles
    - [ ] Barnes–Hut algorithm
    - [ ] collision detection & neighbor finding
      - [ ] spatial dividing (octree, bounding volume heirarchy, etc)
      - [ ] spatial hashing
  - features:
    - [ ] rendering
    - [ ] serialization/deserialization
    - [ ] simd
    - [ ] wasm
    - [ ] parallelization
    - [ ] gpu acceleration

- Examples
  - [ ] cantilever
  - [ ] pile of blocks
  - [ ] cloth
  - [ ] jello block
  - [ ] viscous planet collision
  - [ ] dam break
  - [ ] pile of sand

- Fields
  - [ ] interatomic / intermolecular potentials
    - [X] electrostatic (Coulomb) force
    - [X] general Lennard-Jones potential (Mie potential)
    - [ ] Morse potential
    - [ ] many-body potentials
  - [ ] particle-mesh-method newtonian gravity
  - [ ] SPH newtonian gravity mesh
  - [ ] electromagnetic
  - [ ] material point method mesh
  - [ ] rayleigh dissipation function

- Constraints / Position Based Dynamics
  - [X] compliance, rayleigh dissipation / damping
  - [X] stretching (for young's modulus)
  - [ ] bending
  - [ ] isometric bending?
  - [ ] particle-triangle collisions
  - [X] contact plane collisions (for general purpose shapes & environment)
    - [ ] w/ friction (small steps, survey of pbd, rigid bodies, using average velocity)
  - [X] particle-particle non-penetration
    - [ ] w/ friction (small steps, survey of pbd, rigid bodies, using average velocity)
  - [ ] tetrahedral volume conservation (for bulk modulus) (use neo-hookean bc it doesn't need polar decomposition?)
  - [ ] triangle area conservation
  - [ ] shear (for shear modulus)
  - [ ] neo-hookean soft body (tetrahedral hydrostatic constraint & deviatoric constraint)
  - [ ] closed triangle mesh overpressure
  - [ ] fluid w/ surface tension, cohesion, viscosity
  - [ ] cloth drag (unified particle "cloth and ropes")
  - [ ] shape matching for soft/rigid bodies
  - [ ] generic constraint API
    - see what common behavior can be consolidated into the constraint trait

- Collision Handling
  - [X] collision / impulse forces
  - [ ] friction
  - [ ] viscosity

- Thermodynamics
  - [X] research how to implement a good general macroscopic thermodynamics model
  - [X] is both orientation & size needed for proper thermodyanamics, or only size?
    - neither. thermodynamics is an emergent phenomenon that requires smoothing.
  - [ ] conduction, advection, radiation, friction, collisions
  - [ ] state variables & state equations
    - what is needed in this engine? pos, vel, temp, radius?
    - pressure/volume, temperature/entropy, internal energy, work, heat
    - fields can alter any of these state variables. a field could implement a certain state eqn?
  - [X] thermal expansion
    - increase in temperature -> increase in internal pressure
  - [ ] phase changes

- Compilation Features
  - [ ] serialization of System struct and saving/loading from file (using serde)
  - [ ] load and run a serialized system from the command line (create folder of serialized examples?)
  - [ ] object mesh file loading (for FEM-like analysis)?
  - [ ] parallelize using simd, rayon, bevy_tasks::ParallelIterator, etc
  - [ ] wgpu acceleration

- Rendering
  - [ ] implementable draw method for constraints. ie: drawing contact planes, rod connections, etc
  - [ ] animation baking (both 3d and 2d)
    - [ ] save/serialize System state to file once the animation is completed
  - [ ] color based on radiation/temperature
  - [ ] good 3d rendering
    - [ ] isosurface/isoline rendering?
    - [ ] sph-like smoothing

- Physics
  - [ ] microscopic
    - [ ] molecular dynamics & molecular mechanics
      - [ ] different ensemble constraints
      - [ ] different potentials
      - [ ] solely attractive/repulsive coulomb force
  - [ ] macroscopic
    -[ ] solids (covalent/ionic/metallic bonds)
      - [ ] covalent bond potential, electrostatic attraction
      - [X] xpbd distance constraints w/ compliance (harmonic oscillator)
      - [ ] smoothed-particle-hydrodynamics
      - [ ] material-point-method
    - [ ] liquids & gases (intermolecular forces & hydrogen bonds)
      - [X] Lennard-Jones 6-12, 6-9, 10-12 (Mie), electrostatic attraction
      - [ ] smoothed-particle-hydrodynamics
      - [ ] material-point-method
      - [ ] position-based-fluids
  - [ ] soft bodies
    - [ ] volume conserving, pnuematic, etc
    - [ ] bouncy objects
  - [ ] plastic deformation (break constraint and then create a new one in the new location?)
  - [ ] granular materials

- Notable Others
  - LAMMPS
  - PhysX & Flex
  - Rapier & Parry
  - Bullet
  - PySPH & PHANTOM
  - SPLASH

- Possibly useful crates
  - rendering
    - three-d
    - winit
    - softbuffer
    - tiny_skia
    - bevy
    - plotters
  - performance
    - arrayvec
    - wgpu
    - rayon
    - crossbeam
    - simba & simd
  - data
    - serde
    - bitflags
  - redundancy
    - nalgebra
    - parry
    - rustc-hash