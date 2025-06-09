use crate::object::WgObject;
use crate::object::GpuParticle;
use encase::{ShaderType, StorageBuffer};
use wgcore::kernel::KernelDispatch;
use wgcore::tensor::GpuVector;
use wgcore::Shader;
use wgparry::{dim_shader_defs, substitute_aliases};
use wgpu::util::{BufferInitDescriptor, DeviceExt, DispatchIndirectArgs};
use wgpu::{Buffer, BufferUsages, ComputePass, ComputePipeline, Device};

pub struct GpuSalvaIndirectArgs {
    buffer: Buffer,
}

impl GpuSalvaIndirectArgs {
    pub fn new(device: &Device, dispatch_indirect_args: DispatchIndirectArgs) -> Self {
        Self {
            buffer: device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: dispatch_indirect_args.as_bytes(),
                usage: BufferUsages::INDIRECT | BufferUsages::STORAGE
            })
        }
    }

    pub fn buffer(&self) -> &Buffer { &self.buffer }
}

/// Fluid simulation constants buffer
pub struct GpuConstants {
    buffer: Buffer,
}

impl GpuConstants {
    pub fn init(device: &Device, consts: &SalvaConstants) -> Self {
        let byte_len = consts.size().get() as usize;
        let mut bytes = Vec::with_capacity(byte_len);
        StorageBuffer::new(&mut bytes)
            .write(consts)
            .unwrap();
        Self {
            buffer: device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytes.as_slice(),
                usage: BufferUsages::UNIFORM
            })
        }
    }

    pub fn buffer(&self) -> &Buffer { &self.buffer }
}

/// Buffer layout of the fluid simulation constants.
#[derive(ShaderType, Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct SalvaConstants {
    pub particle_radius: f32,
    pub kernel_radius: f32,
}

impl SalvaConstants {
    pub fn new(particle_radius: f32, smoothing_factor: f32) -> Self {
        Self {
            particle_radius,
            kernel_radius: particle_radius * smoothing_factor * 2.
        }
    }
}

// TODO: pub struct GpuBoundary

#[derive(Shader)]
#[shader(
    derive(WgObject),
    src = "liquid_world.wgsl",
    src_fn = "substitute_aliases",
    shader_defs = "dim_shader_defs"
)]
pub struct WgLiquidWorld;

wgcore::test_shader_compilation!(WgLiquidWorld, wgcore, wgparry::dim_shader_defs());

#[derive(Shader)]
#[shader(
    derive(WgLiquidWorld),
    src = "grid_insertion.wgsl",
    src_fn = "substitute_aliases",
    shader_defs = "dim_shader_defs",
    composable = false,
)]
/// Compute shader for inserting fluid particles into a GpuGrid
pub struct WgGridInsertion {
    pub grid_insertion: ComputePipeline
}

impl WgGridInsertion {
    const WORKGROUP_SIZE: u32 = 64;

    /// Dispatch an invocation of [`WgGridInsertion::grid_insertion`] to insert fluid/boundary [`GpuParticle`]s
    /// into the given [`GpuParticleGrid`].
    ///
    /// `narrow_phase_indirect_args` will be updated accordingly for narrow phase collision detection
    /// where there will be one invocation for every grid cell.
    ///
    /// `particles_per_invocation` may not be fully accurate since `particles.len() / particles_per_invocation`
    /// may not be a multiple of [`WgGridInsertion::WORKGROUP_SIZE`]. Extra kernel invocations may
    /// be made.
    pub fn dispatch(
        &self,
        device: &Device,
        pass: &mut ComputePass,
        constants: &GpuConstants,
        particles: &GpuVector<GpuParticle>,
        narrow_phase_indirect_args: &GpuSalvaIndirectArgs,
        particles_per_invocation: u32
    ) {
        let num_invocations = particles.len() as u32 / particles_per_invocation;
        let num_workgroups = num_invocations.div_ceil(Self::WORKGROUP_SIZE);
        KernelDispatch::new(device, pass, &self.grid_insertion)
            .bind0([
                constants.buffer(),
                particles.buffer(),
                narrow_phase_indirect_args.buffer()
            ])
            .dispatch(num_workgroups);
    }
}

wgcore::test_shader_compilation!(WgGridInsertion, wgcore, wgparry::dim_shader_defs());
