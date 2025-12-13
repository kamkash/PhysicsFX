struct Instance {
    position: vec2<f32>,
    velocity: vec2<f32>,
    scale: f32,
    rotation: f32,
    uv: vec2<f32>,
};

@group(0) @binding(0)
var<storage, read_write> instances: array<Instance>;

// Compute Shader
@compute @workgroup_size(64)
fn update_instances(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= arrayLength(&instances)) {
        return;
    }

    // Simple physics update: pos += vel * dt (assuming fixed dt usually, or passed via uniform)
    // For now, let's just make them move based on velocity.
    // To make it interesting without a uniform DT yet, we'll assume a small factor.
    let dt = 0.016; 
    let instance = instances[index];
    
    // Update position
    var new_pos = instance.position + instance.velocity * dt;

    // Bounce off walls (assuming -1.0 to 1.0 clip space range for simplicity in logic)
    // In a real app, this logic might be more complex or depend on world size.
    var new_vel = instance.velocity;
    if (new_pos.x < -1.0 || new_pos.x > 1.0) {
        new_vel.x = -new_vel.x;
        new_pos.x = clamp(new_pos.x, -1.0, 1.0);
    }
    if (new_pos.y < -1.0 || new_pos.y > 1.0) {
        new_vel.y = -new_vel.y;
        new_pos.y = clamp(new_pos.y, -1.0, 1.0);
    }

    // Write back
    instances[index].position = new_pos;
    instances[index].velocity = new_vel;
    // Rotate slightly
    instances[index].rotation += 0.02;
}

// Vertex Shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct InstanceInput {
    @location(2) i_position: vec2<f32>,
    @location(3) i_velocity: vec2<f32>, // Unused in VS, but part of buffer stride
    @location(4) i_scale: f32,
    @location(5) i_rotation: f32,
    @location(6) i_uv: vec2<f32>, // UV offset/scale could go here, for now just passing through or ignoring
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    // scale
    let scaled_pos = model.position * instance.i_scale;

    // rotate (2D only Z-axis rotation)
    let c = cos(instance.i_rotation);
    let s = sin(instance.i_rotation);
    let rotated_pos = vec3<f32>(
        scaled_pos.x * c - scaled_pos.y * s,
        scaled_pos.x * s + scaled_pos.y * c,
        scaled_pos.z
    );

    // translate
    let world_pos = vec3<f32>(
        rotated_pos.x + instance.i_position.x,
        rotated_pos.y + instance.i_position.y,
        rotated_pos.z
    );

    var out: VertexOutput;
    out.tex_coords = model.tex_coords; // + instance.i_uv if we wanted UV scrolling
    out.clip_position = vec4<f32>(world_pos, 1.0);
    return out;
}

// Fragment Shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
