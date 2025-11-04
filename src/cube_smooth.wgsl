struct Uniforms {
    model_view: mat4x4<f32>,
    model_view_projection: mat4x4<f32>,
    normal: mat3x3<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let light_source = vec4<f32>(2.0, 2.0, 20.0, 0.0);

    // Transform position to clip space
    out.clip_position = uniforms.model_view_projection * vec4<f32>(in.position, 1.0);

    // Transform normal to eye/view space
    let eye_normal = uniforms.normal * in.normal;

    // Transform position to view space with perspective correction
    let position4 = uniforms.model_view * vec4<f32>(in.position, 1.0);
    let position3 = position4.xyz / position4.w;

    // Calculate light direction from vertex to light source
    let light_dir = normalize(light_source.xyz - position3);

    // Calculate diffuse lighting
    let diff = max(0.0, dot(eye_normal, light_dir));
    out.color = diff * in.color;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Use GPU interpolation of vertex colors
    return vec4<f32>(in.color, 1.0);
}
