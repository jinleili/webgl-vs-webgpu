use crate::{
    camera::Camera,
    comparative_scenario::{CompScenario, ThousandsEntity},
    ui::EguiLayer,
    util::matrix_helper,
    util::BufferObj,
    SceneUniform, DEPTH_FORMAT,
};
use app_surface::{AppSurface, SurfaceFrame};
use raw_window_handle::HasRawDisplayHandle;
use std::{iter, mem::size_of};
use wgpu::TextureView;
use winit::{dpi::PhysicalSize, window::WindowId};

pub struct App {
    pub(crate) app: AppSurface,
    pub(crate) egui_layer: EguiLayer,
    pub(crate) canvas_size: app_surface::ViewSize,
    pub(crate) depth_view: TextureView,
    pub(crate) scene_uniform_buf: BufferObj,
    comp_scenario: Option<Box<dyn CompScenario>>,
}

impl App {
    pub async fn new(app: AppSurface, event_loop: &dyn HasRawDisplayHandle) -> Self {
        let mut app = app;
        // WebGL 无法使用 MSAA
        // WebGL 无法使用 view_formats, 且 egui 等偏向于使用 none-sRGB
        // 在 app-surface 中对 WebGL 后端做了不设置 view_formats 的处理
        let format = {
            let remove_srgb = app.config.format.remove_srgb_suffix();
            app.sdq.update_config_format(remove_srgb);

            remove_srgb
        };

        // egui
        let egui_layer = EguiLayer::new(&app, event_loop, format).await;

        let canvas_size = app.view_size();
        let depth_view = Self::create_depth_tex(&app);

        let buf_size = size_of::<SceneUniform>() as wgpu::BufferAddress;
        let scene_uniform_buf = BufferObj::create_empty_uniform_buffer(
            &app.device,
            buf_size,
            buf_size,
            false,
            Some("scene buffer"),
        );
        Self::cal_scene_uniform(&app, &scene_uniform_buf);

        let mut instance = Self {
            app,
            egui_layer,
            canvas_size,
            depth_view,
            scene_uniform_buf,
            comp_scenario: None,
        };
        let comp_scenario = ThousandsEntity::new(&instance).await;
        instance.comp_scenario = Some(Box::new(comp_scenario));

        instance
    }

    pub fn get_adapter_info(&self) -> wgpu::AdapterInfo {
        self.app.adapter.get_info()
    }

    pub fn get_view_mut(&mut self) -> &mut winit::window::Window {
        &mut self.app.view
    }

    pub fn current_window_id(&self) -> WindowId {
        self.app.view.id()
    }

    pub fn resize(&mut self, size: &PhysicalSize<u32>) {
        if self.app.config.width == size.width && self.app.config.height == size.height {
            return;
        }
        // 调整画布尺寸
        self.app.resize_surface();
        Self::cal_scene_uniform(&self.app, &self.scene_uniform_buf);

        // 重新生成与画布尺寸有关的纹理
        self.depth_view = Self::create_depth_tex(&self.app);
        self.egui_layer.resize(&self.app);
        self.canvas_size = self.app.view_size();
    }

    pub fn request_redraw(&mut self) {
        self.app.view.request_redraw();
    }

    pub fn render(&mut self) {
        // WebGL 不能使用 view_formats 重新解释能力
        let view_format = Some(self.app.config.format);
        let (output, frame_view) = self.app.get_current_frame_view(view_format);
        let mut encoder = self
            .app
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // egui ui 刷新
        let egui_cmd_buffers = self.egui_layer.refresh_ui(&self.app, &mut encoder);
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("main rpass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.15,
                            g: 0.15,
                            b: 0.15,
                            a: 0.15,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });

            if self.comp_scenario.is_some() {
                self.comp_scenario
                    .as_deref_mut()
                    .unwrap()
                    .draw_by_pass(&self.app, &mut rpass);
            }

            // egui ui 渲染
            self.egui_layer.draw_by_pass(&self.app, &mut rpass);
        }

        // 提交命令
        self.app.queue.submit(
            egui_cmd_buffers
                .into_iter()
                .chain(iter::once(encoder.finish())),
        );
        output.present();
    }

    fn cal_scene_uniform(app: &AppSurface, buf: &BufferObj) {
        let view_port = glam::Vec2::new(app.config.width as f32, app.config.height as f32);
        let (proj_matrix, mut view_mat, factor) = matrix_helper::perspective_mvp(view_port);
        let (ortho_matrix, _) = matrix_helper::ortho_mvp(view_port);

        let x_pixel_to_ndc = factor.sx / app.config.width as f32 * 2.0;
        let y_pixel_to_ndc = factor.sy / app.config.height as f32 * 2.0;
        let z_pixel_to_ndc = if factor.translate_z.eq(&1.0) {
            y_pixel_to_ndc
        } else {
            x_pixel_to_ndc
        };
        let view_scale = glam::Vec3::new(x_pixel_to_ndc, y_pixel_to_ndc, z_pixel_to_ndc);
        view_mat *= glam::Mat4::from_scale(view_scale);

        let width = app.config.width as f32;
        let height = app.config.height as f32;
        let camera = Camera::new(width, height);
        let camera_mat = camera.calc_matrix();
        let view_camera = camera_mat * view_mat;
        let view_proj_mat = proj_matrix * view_camera;

        let view_proj_uniform = SceneUniform {
            view_mat: view_camera.to_cols_array_2d(),
            proj_mat: proj_matrix.to_cols_array_2d(),
            view_proj: view_proj_mat.to_cols_array_2d(),
            view_ortho: (ortho_matrix * view_camera).to_cols_array_2d(),
            camera_pos: glam::Vec4::from((camera.position, 1.0)).to_array(),
            viewport_pixels: [width, height],
            _padding: [0.0; 2],
        };
        app.queue
            .write_buffer(&buf.buffer, 0, bytemuck::bytes_of(&view_proj_uniform));
    }

    fn create_depth_tex(app: &AppSurface) -> wgpu::TextureView {
        let depth_texture = app.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: app.config.width,
                height: app.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            label: None,
            view_formats: &[],
        });
        depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }
}
