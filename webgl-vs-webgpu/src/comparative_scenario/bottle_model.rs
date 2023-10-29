use crate::{geometries, util::vertex::PosNormalUv};
use app_surface::AppSurface;
use std::mem::size_of;
use wgpu::{util::DeviceExt, Buffer};

pub struct BottleModel {
    pub vertex_buf_list: Vec<Buffer>,
    pub index_buf_list: Vec<Buffer>,
    pub vertex_count: u32,
    pub index_count: u32,
}

impl BottleModel {
    pub async fn new(app: &AppSurface, buffer_count: usize) -> Self {
        // 模型网格
        let bottle = geometries::bottle::bottle(250., 175., 100.);
        let vertex_count = bottle.0.attributes().len();
        let size = (size_of::<PosNormalUv>() * vertex_count) as _;

        let mut vertex_buf_list: Vec<Buffer> = Vec::with_capacity(buffer_count);
        let mut index_buf_list: Vec<Buffer> = Vec::with_capacity(buffer_count);
        for _ in 0..buffer_count {
            let vertex_buf = app.device.create_buffer(&wgpu::BufferDescriptor {
                size,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                label: Some("mesh buffer"),
                mapped_at_creation: false,
            });
            app.queue
                .write_buffer(&vertex_buf, 0, bytemuck::cast_slice(&bottle.0.attributes()));
            let index_buf = app
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("index buffer"),
                    contents: bytemuck::cast_slice(&bottle.1),
                    usage: wgpu::BufferUsages::INDEX,
                });
            vertex_buf_list.push(vertex_buf);
            index_buf_list.push(index_buf);
        }

        let index_count = bottle.1.len() as u32;

        Self {
            vertex_buf_list,
            index_buf_list,
            vertex_count: vertex_count as u32,
            index_count,
        }
    }
}
