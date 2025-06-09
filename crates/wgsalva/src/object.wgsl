#define_import_path wgsalva::object

/// A fluid object.
///
/// A fluid object is composed of movable particles with additional properties like viscosity.
struct Fluid {
    // TODO: pub nonpressure_forces: bitflags? fixed array/buffer ?
    /// The rest density of this fluid.
    density: f32,
    /// The index in the GPU particle buffer where this fluid's particles start.
    ///
    /// The index range in the buffer where this fluid's particles are stored is
    /// [`particle_buffer_start`, `particle_buffer_end`]
    particle_buffer_start: u32,
    /// The index in the GPU particle buffer where this fluid's particles end.
    ///
    /// The index range in the buffer where this fluid's particles are stored is
    /// [`particle_buffer_start`, `particle_buffer_end`]
    particle_buffer_end: u32,
    // TODO: pub interaction_groups
}

/// A particle belonging to a fluid or boundary object.
struct Particle {
    position: Vector,
    velocity: Vector,
    acceleration: Vector,
    /// The index referencing the object (fluid / boundary) this particle belongs to.
    object_idx: u32,
    /// The position of this particle in the particle grid
#if DIM == 2
    grid_pos: vec2<i32>,
#else
    grid_pos: vec3<i32>,
#endif
}

// TODO: pub struct Boundary