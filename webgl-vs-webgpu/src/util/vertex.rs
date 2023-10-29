#[allow(dead_code)]
use serde::{Deserialize, Serialize};

pub trait Vertex {
    fn bit32_count() -> u32;
    fn vertex_attributes(offset: u32) -> Vec<wgpu::VertexAttribute>;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexEmpty {}
impl Vertex for VertexEmpty {
    fn vertex_attributes(_offset: u32) -> Vec<wgpu::VertexAttribute> {
        vec![]
    }
    fn bit32_count() -> u32 {
        0
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PosOnly {
    pub pos: [f32; 3],
}

impl Vertex for PosOnly {
    fn vertex_attributes(offset: u32) -> Vec<wgpu::VertexAttribute> {
        vec![wgpu::VertexAttribute {
            shader_location: offset,
            format: wgpu::VertexFormat::Float32x3,
            offset: 0,
        }]
    }
    fn bit32_count() -> u32 {
        3
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PosColor {
    pub pos: [f32; 3],
    pub color: [f32; 4],
}

impl Vertex for PosColor {
    fn vertex_attributes(offset: u32) -> Vec<wgpu::VertexAttribute> {
        vec![
            wgpu::VertexAttribute {
                shader_location: offset,
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
            },
            wgpu::VertexAttribute {
                shader_location: offset + 1,
                format: wgpu::VertexFormat::Float32x4,
                offset: 4 * 3,
            },
        ]
    }
    fn bit32_count() -> u32 {
        7
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable, Serialize, Deserialize)]
pub struct PosNormalUv {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex for PosNormalUv {
    fn vertex_attributes(offset: u32) -> Vec<wgpu::VertexAttribute> {
        vec![
            wgpu::VertexAttribute {
                shader_location: offset,
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
            },
            wgpu::VertexAttribute {
                shader_location: offset + 1,
                format: wgpu::VertexFormat::Float32x3,
                offset: 4 * 3,
            },
            wgpu::VertexAttribute {
                shader_location: offset + 2,
                format: wgpu::VertexFormat::Float32x2,
                offset: 4 * 6,
            },
        ]
    }
    fn bit32_count() -> u32 {
        8
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PosNormalUvIndex {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub index: u32,
}

impl Vertex for PosNormalUvIndex {
    fn vertex_attributes(offset: u32) -> Vec<wgpu::VertexAttribute> {
        vec![
            wgpu::VertexAttribute {
                shader_location: offset,
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
            },
            wgpu::VertexAttribute {
                shader_location: offset + 1,
                format: wgpu::VertexFormat::Float32x3,
                offset: 4 * 3,
            },
            wgpu::VertexAttribute {
                shader_location: offset + 2,
                format: wgpu::VertexFormat::Float32x2,
                offset: 4 * 6,
            },
            wgpu::VertexAttribute {
                shader_location: offset + 3,
                format: wgpu::VertexFormat::Uint32,
                offset: 4 * 8,
            },
        ]
    }
    fn bit32_count() -> u32 {
        9
    }
}

/// 变换工具的顶点数据
/// 为了方便，直接在顶点数据中存储了默认色与选中色
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ToolColor {
    pub display_color: [f32; 4],
    pub pick_color: [f32; 4],
}

impl Vertex for ToolColor {
    fn vertex_attributes(offset: u32) -> Vec<wgpu::VertexAttribute> {
        vec![
            wgpu::VertexAttribute {
                shader_location: offset,
                format: wgpu::VertexFormat::Float32x4,
                offset: 0,
            },
            wgpu::VertexAttribute {
                shader_location: offset + 1,
                format: wgpu::VertexFormat::Float32x4,
                offset: 4 * 4,
            },
        ]
    }
    fn bit32_count() -> u32 {
        8
    }
}
