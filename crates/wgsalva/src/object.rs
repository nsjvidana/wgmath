use encase::ShaderType;
use wgcore::Shader;
use wgparry::math::Vector;
use wgparry::{dim_shader_defs, substitute_aliases};

/// A fluid object.
///
/// A fluid object is composed of movable particles with additional properties like viscosity.
#[derive(ShaderType)]
pub struct GpuFluid {
    // TODO: pub nonpressure_forces: bitflags? fixed array?
    /// The rest density of this fluid.
    pub density: f32,
    /// The index in the GPU particle buffer where this fluid's particles start.
    ///
    /// The index range in the buffer where this fluid's particles are stored is
    /// [`particle_buffer_start`, `particle_buffer_end`]
    pub particle_buffer_start: u32,
    /// The index in the GPU particle buffer where this fluid's particles end.
    ///
    /// The index range in the buffer where this fluid's particles are stored is
    /// [`particle_buffer_start`, `particle_buffer_end`]
    pub particle_buffer_end: u32,
    // TODO: pub interaction_groups
}

/// A particle belonging to a fluid or boundary object.
#[derive(ShaderType, Copy, Clone, PartialEq, Default)]
#[repr(C)]
pub struct GpuParticle {
    pub position: Vector<f32>,
    pub velocity: Vector<f32>,
    pub acceleration: Vector<f32>,
    /// The index referencing the object (fluid / boundary) this particle belongs to.
    pub object_idx: u32,
    pub grid_pos: Vector<i32>,
}

#[derive(Shader)]
#[shader(
    src = "object.wgsl",
    src_fn = "substitute_aliases",
    shader_defs = "dim_shader_defs"
)]
pub struct WgObject;

wgcore::test_shader_compilation!(WgObject, wgcore, wgparry::dim_shader_defs());
