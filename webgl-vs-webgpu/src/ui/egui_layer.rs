use crate::DEPTH_FORMAT;

use super::ControlPanel;
use app_surface::AppSurface;
use egui::ClippedPrimitive;
use egui_wgpu::renderer::ScreenDescriptor;
use raw_window_handle::HasRawDisplayHandle;

pub struct EguiLayer {
    pub ctx: egui::Context,
    pub ctrl_panel: ControlPanel,
    screen_descriptor: ScreenDescriptor,
    egui_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
    clipped_primitives: Option<Vec<ClippedPrimitive>>,
}

#[allow(unused)]
impl EguiLayer {
    pub async fn new(
        app: &AppSurface,
        event_loop: &dyn HasRawDisplayHandle,
        format: wgpu::TextureFormat,
    ) -> Self {
        let ctx = egui::Context::default();
        setup_custom_fonts(&ctx);
        let ctrl_panel = ControlPanel::new(app, &ctx, app.adapter.get_info().backend.clone());
        let screen_descriptor = egui_wgpu::renderer::ScreenDescriptor {
            size_in_pixels: [app.config.width, app.config.height],
            pixels_per_point: app.scale_factor,
        };
        let mut egui_state = egui_winit::State::new(event_loop);
        egui_state.set_pixels_per_point(app.scale_factor);
        let egui_renderer = egui_wgpu::Renderer::new(&app.device, format, Some(DEPTH_FORMAT), 1);

        Self {
            ctx,
            ctrl_panel,
            screen_descriptor,
            egui_state,
            egui_renderer,
            clipped_primitives: None,
        }
    }

    pub fn on_ui_event(&mut self, event: &winit::event::WindowEvent<'_>) {
        let _response = self.egui_state.on_event(&self.ctx, event);
    }

    // 更新帧信息
    pub fn on_new_frame(&mut self, now: f64, frame_time: f32, cpu_usage_time: f32) {
        self.ctrl_panel
            .frame_history
            .on_new_frame(now, frame_time, cpu_usage_time);
    }

    pub fn resize(&mut self, app: &AppSurface) {
        self.screen_descriptor = egui_wgpu::renderer::ScreenDescriptor {
            size_in_pixels: [app.config.width, app.config.height],
            pixels_per_point: app.scale_factor,
        };
    }

    pub fn refresh_ui(
        &mut self,
        app: &AppSurface,
        encoder: &mut wgpu::CommandEncoder,
    ) -> Vec<wgpu::CommandBuffer> {
        let raw_input = self.egui_state.take_egui_input(&app.view);
        let full_output = self.ctx.run(raw_input, |ctx| {
            self.ctrl_panel.ui_contents(ctx);
        });
        let clipped_primitives = self.ctx.tessellate(full_output.shapes);
        let textures_delta = full_output.textures_delta;

        let egui_cmd_bufs = {
            for (id, image_delta) in &textures_delta.set {
                self.egui_renderer
                    .update_texture(&app.device, &app.queue, *id, image_delta);
            }
            self.egui_renderer.update_buffers(
                &app.device,
                &app.queue,
                encoder,
                &clipped_primitives,
                &self.screen_descriptor,
            )
        };
        for id in &textures_delta.free {
            self.egui_renderer.free_texture(id);
        }
        self.clipped_primitives = Some(clipped_primitives);

        egui_cmd_bufs
    }

    pub fn draw_by_pass<'a, 'b: 'a>(&'b self, _app: &AppSurface, rpass: &mut wgpu::RenderPass<'b>) {
        // egui ui 渲染
        if let Some(clipped_primitives) = self.clipped_primitives.as_ref() {
            self.egui_renderer
                .render(rpass, clipped_primitives, &self.screen_descriptor);
        }
    }
}

const ZH_TINY: &str = "zh";

pub(crate) fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        ZH_TINY.to_owned(),
        egui::FontData::from_static(include_bytes!("../../../assets/fonts/PingFangTiny.ttf")),
    );
    fonts
        .families
        .insert(egui::FontFamily::Proportional, vec![ZH_TINY.to_owned()]);

    // Put my font as last fallback for monospace:
    // 如果没有这项设置，`syntax_highlighting::code_view_ui` 无法渲染任何字符
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push(ZH_TINY.to_owned());

    ctx.set_fonts(fonts);
}
