#include "struct/scene_uniform.wgsl"
#include "func/microfacet_lighting.wgsl"

struct ModelUniform {
    model_mat: mat4x4f,
    albedo: vec4f,
}

@group(0) @binding(0) var<uniform> scene: SceneUniform;
@group(0) @binding(1) var<uniform> model_uniform: ModelUniform;
@group(0) @binding(2) var<uniform> material: Material;

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) uv: vec2f,
    @location(1) world_pos: vec3f,
    @location(2) world_normal: vec3f,
    @location(3) @interpolate(flat) albedo: vec4f,
};

@vertex
fn vs_main(
    @location(0) pos: vec3f,
    @location(1) normal: vec3f,
    @location(2) uv: vec2f,
) -> VertexOutput {
    let m = model_uniform.model_mat;
    let world_pos = scene.view_mat * m * vec4f(pos, 1.0);
    var out: VertexOutput;
    out.position = scene.proj_mat * world_pos;
    out.uv = uv;
    out.world_pos = world_pos.xyz;
    out.world_normal = mat3x3f(m[0].xyz, m[1].xyz, m[2].xyz) * normal;
    out.albedo = model_uniform.albedo;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    // 修改不同实例的材质颜色
    var new_material: Material;
    new_material.albedo = in.albedo;
    new_material.roughness = material.roughness;
    new_material.reflectance = material.reflectance;
    new_material.ambient_ratio = material.ambient_ratio;
    new_material.background_ratio = material.background_ratio;

    // 固定光源
    var light: Light;
    light.pos = vec4f(2., 3.5, 8., 1.);
    light.color = vec4f(1.);
    light.ty = vec4u(0u);

    let camera_dir = normalize(scene.camera_pos.xyz - in.world_pos);
    let normal = normalize(in.world_normal);

    // 着色计算
    var pre_color = vec3f(0.);
    pre_color = pre_color + microfacet_color(in.world_pos, normal, light, camera_dir, new_material);
    pre_color = clamp(pre_color, vec3f(0.0), vec3f(1.0));
    pre_color = background_correction(pre_color, vec3f(0.24), new_material);
    pre_color = ambient_correction(pre_color, new_material);

    return vec4f(pre_color, 1.0);
}