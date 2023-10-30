use bytemuck::{Pod, Zeroable};

mod app;
pub use app::App;
pub mod run;

mod camera;
mod color;
mod comparative_scenario;
mod geometries;
mod node;
mod ui;
mod util;

pub static DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct SceneUniform {
    /// 仅视图矩阵，没有预乘投影矩阵
    view_mat: [[f32; 4]; 4],
    /// 仅投影矩阵，没有预乘视图矩阵
    proj_mat: [[f32; 4]; 4],
    view_proj: [[f32; 4]; 4],
    // 法线矩阵
    // normal: [[f32; 4]; 4],
    view_ortho: [[f32; 4]; 4],
    // 相机位置
    camera_pos: [f32; 4],
    viewport_pixels: [f32; 2],
    _padding: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct ModelUniformData {
    // 使用 1024, Chrome 上 webgl 报错，Caused by:
    //     In Device::create_bind_group
    //     Buffer binding 1 range 65536 exceeds `max_*_buffer_binding_size` limit 16384
    // model_mat_list: [[[f32; 4]; 4]; 128],
    model_mat: [[f32; 4]; 4],
    albedo: [f32; 4],
}

impl Default for ModelUniformData {
    fn default() -> Self {
        ModelUniformData {
            model_mat: glam::Mat4::IDENTITY.to_cols_array_2d(),
            albedo: [0.2, 0.3, 0.5, 1.0],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Material {
    /// 基色
    pub albedo: [f32; 4],
    /// 表面粗糙度.
    pub roughness: f32,
    /// 高光反射率.
    pub reflectance: f32,
    pub ambient_ratio: f32,
    pub background_ratio: f32,
}

impl Default for Material {
    fn default() -> Material {
        Material {
            albedo: [1.0, 1.0, 1.0, 1.0],
            roughness: 0.5,
            reflectance: 0.2,
            ambient_ratio: 0.2,
            background_ratio: 0.05,
        }
    }
}