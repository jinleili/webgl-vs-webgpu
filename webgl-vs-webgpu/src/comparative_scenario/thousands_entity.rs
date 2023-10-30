use super::{bottle_model::BottleModel, CompScenario, PSO_COUNT};
use crate::{
    color::{unpack_u32_to_rgba_f32, VISION_COLORS},
    node::{BindGroupData, BindGroupSetting},
    util::{
        vertex::{PosNormalUv, Vertex},
        BufferObj,
    },
    App, Material, ModelUniformData,
};
use app_surface::AppSurface;
use rand::Rng;
use wgpu::{MultisampleState, RenderPipeline, ShaderStages};

#[allow(unused)]
pub struct ThousandsEntity {
    model_uniform_buf: Vec<BufferObj>,
    model_uniform_data: Vec<ModelUniformData>,
    model_rotation_data: Vec<(glam::Vec3, f32, f32)>,
    material_uniform_buf: BufferObj,
    bg_setting_list: Vec<BindGroupSetting>,
    pipeline_list: Vec<RenderPipeline>,
    model: BottleModel,
}

impl ThousandsEntity {
    pub async fn new(app: &App) -> Self {
        let device = &app.app.device;

        // 模型 uniform
        let mut model_uniform_data: Vec<ModelUniformData> = Vec::with_capacity(PSO_COUNT);
        let mut model_rotation_data: Vec<(glam::Vec3, f32, f32)> = Vec::with_capacity(PSO_COUNT);
        let mut model_uniform_buf: Vec<BufferObj> = Vec::with_capacity(PSO_COUNT);

        let mut rng = rand::thread_rng();
        let config = &app.app.config;
        let mut max_viewport = if config.width > config.height {
            config.height as f32
        } else {
            config.width as f32
        };
        max_viewport *= 2.5;
        for _ in 0..PSO_COUNT {
            let display_color = VISION_COLORS[(rng.gen::<f32>() * 29.0).floor() as usize];
            let mut a_model_uniform = ModelUniformData::default();
            a_model_uniform.albedo = unpack_u32_to_rgba_f32(display_color);

            let random_x: f32 = (rng.gen::<f32>() - 0.5) * max_viewport;
            let random_y: f32 = (rng.gen::<f32>() - 0.5) * max_viewport;
            let model_mat = glam::Mat4::from_translation(glam::Vec3::new(
                random_x,
                random_y,
                (rng.gen::<f32>() - 0.5) * max_viewport * 0.5,
            ));
            let scale = rng.gen::<f32>() * 0.5 + 0.25;
            let scale_mat = glam::Mat4::from_scale(glam::Vec3::new(scale, scale, scale));
            a_model_uniform.model_mat = (model_mat * scale_mat).to_cols_array_2d();
            let rotation_axis =
                glam::Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()).normalize();
            model_rotation_data.push((rotation_axis, rng.gen::<f32>(), rng.gen::<f32>() + 1.));

            let a_model_buf =
                BufferObj::create_uniform_buffer(device, &a_model_uniform, Some("model buffer"));
            model_uniform_data.push(a_model_uniform);
            model_uniform_buf.push(a_model_buf);
        }

        // 材质信息
        let material_data = Material {
            albedo: [0.3, 0.4, 0.5, 1.],
            ..Default::default()
        };
        let material_uniform_buf =
            BufferObj::create_uniform_buffer(device, &material_data, Some("material buffer"));

        // let polygon_shader =
        //     crate::create_shader_module(device, "polygon", Some("polygon shader")).await;
        let polygon_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("polygon shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../../../assets/preprocessed-wgsl/polygon.wgsl").into(),
            ),
        });

        let mut bg_setting_list: Vec<BindGroupSetting> = Vec::with_capacity(PSO_COUNT);
        let mut pipeline_list: Vec<RenderPipeline> = Vec::with_capacity(PSO_COUNT);
        for i in 0..PSO_COUNT {
            let bg_data = BindGroupData {
                uniforms: vec![
                    &app.scene_uniform_buf,
                    &model_uniform_buf[i],
                    &material_uniform_buf,
                ],
                visibilitys: vec![
                    ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                    ShaderStages::VERTEX,
                    ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ],
                ..Default::default()
            };
            let bg_setting = BindGroupSetting::new(device, &bg_data);
            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bg_setting.bind_group_layout],
                push_constant_ranges: &[],
            });

            // PSO
            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("polygon pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &polygon_shader,
                    entry_point: "vs_main",
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<PosNormalUv>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &PosNormalUv::vertex_attributes(0),
                    }],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &polygon_shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: app.app.config.format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    cull_mode: Some(wgpu::Face::Back),
                    ..Default::default()
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: crate::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: MultisampleState::default(),
                multiview: None,
            });
            bg_setting_list.push(bg_setting);
            pipeline_list.push(pipeline);
        }

        let model = BottleModel::new(&app.app, PSO_COUNT).await;
        Self {
            material_uniform_buf,
            model_uniform_data,
            model_rotation_data,
            model_uniform_buf,
            bg_setting_list,
            pipeline_list,
            model,
        }
    }
}

impl CompScenario for ThousandsEntity {
    fn draw_by_pass<'a, 'b: 'a>(&'b mut self, app: &AppSurface, rpass: &mut wgpu::RenderPass<'b>) {
        // 先更新数据
        for i in 0..PSO_COUNT {
            let original_mat = self.model_uniform_data[i as usize];
            let model_mat = glam::Mat4::from_cols_array_2d(&original_mat.model_mat);
            let rotation_data = &mut self.model_rotation_data[i as usize];
            rotation_data.1 += 0.05 * rotation_data.2;
            let rotation_mat = glam::Mat4::from_axis_angle(rotation_data.0, rotation_data.1);
            let a_model_mat = (model_mat * rotation_mat).to_cols_array_2d();
            app.queue.write_buffer(
                &self.model_uniform_buf[i].buffer,
                0,
                bytemuck::bytes_of(&a_model_mat),
            );
        }

        // 再绘制
        for i in 0..PSO_COUNT {
            rpass.set_pipeline(&self.pipeline_list[i]);
            rpass.set_bind_group(0, &self.bg_setting_list[i].bind_group, &[]);
            rpass.set_index_buffer(
                self.model.index_buf_list[i].slice(..),
                wgpu::IndexFormat::Uint32,
            );
            rpass.set_vertex_buffer(0, self.model.vertex_buf_list[i].slice(..));
            rpass.draw_indexed(0..self.model.index_count, 0, 0..1);
        }
    }
}
