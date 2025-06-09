#define_import_path wgsalva::grid_insertion

#import wgsalva::liquid_world as LiquidWorld
#import wgsalva::object as Object

@group(0) @binding(0)
var<uniform> consts: LiquidWorld::SalvaConstants;

@group(0) @binding(1)
var<storage, read_write> particles: array<Object::Particle>;

struct DispatchIndirectArgs {
    x: u32,
    y: u32,
    z: u32,
}

@group(0) @binding(2)
var<storage, read_write> narrow_phase_indirect_args: DispatchIndirectArgs;

const WORKGROUP_SIZE: u32 = 64u;
@compute @workgroup_size(WORKGROUP_SIZE)
fn grid_insertion(
    @builtin(global_invocation_id) invocation_id: vec3<u32>,
    @builtin(num_workgroups) num_workgroups: vec3<u32>
) {
    let num_particles = arrayLength(&particles);
    let invocation_count = num_workgroups.x * WORKGROUP_SIZE;
    let particles_per_invocation = (num_particles / invocation_count) + u32(num_particles % invocation_count != 0); // div ceil
    let start_idx = invocation_id.x * particles_per_invocation;
    for (var i = 0u; i < particles_per_invocation; i++) {
        let j = min(i + start_idx, num_particles - 1);
        #if DIM == 2
            particles[j].grid_pos = vec2i(particles[j].position / consts.kernel_radius);
        #else
            particles[j].grid_pos = vec3i(particles[j].position / consts.kernel_radius);
        #endif
    }

    // TODO: set indirect args X to number of grid cells
    if invocation_id.x == 0 {
        narrow_phase_indirect_args.x = 1u;
        narrow_phase_indirect_args.y = 1u;
        narrow_phase_indirect_args.z = 1u;
    }
}