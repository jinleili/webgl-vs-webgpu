use app_surface::AppSurface;
use egui::{Color32, Context, Ui};

use super::frame_history::FrameHistory;

pub struct ControlPanel {
    panel_frame: egui::Frame,
    window_size: egui::emath::Vec2,
    pub model_count: u32,
    pub frame_history: FrameHistory,
    backend: wgpu::Backend,
}

impl ControlPanel {
    pub fn new(app: &AppSurface, egui_ctx: &Context, backend: wgpu::Backend) -> Self {
        let margin = 8.0;
        let panel_width = 320.0;
        let panel_height = app.config.height as f32 / app.scale_factor - margin * 2.0;

        // 实测出来的数值，避免圆角被裁剪
        let window_size: egui::emath::Vec2 = [panel_width - 26.0, panel_height - 12.].into();

        let mut bg = egui_ctx.style().visuals.window_fill();
        bg = egui::Color32::from_rgba_premultiplied(bg.r(), bg.g(), bg.b(), 230);
        let panel_frame = egui::Frame {
            fill: bg,
            rounding: 10.0.into(),
            stroke: egui_ctx.style().visuals.widgets.noninteractive.fg_stroke,
            outer_margin: 0.5.into(), // so the stroke is within the bounds
            inner_margin: 12.0.into(),
            ..Default::default()
        };

        Self {
            panel_frame,
            window_size,
            model_count: 10,
            frame_history: FrameHistory::default(),
            backend,
        }
    }

    pub fn ui_contents(&mut self, ctx: &Context) {
        let window = egui::Window::new("帧信息")
            .id(egui::Id::new("webgl_vs_webgpu_window"))
            .resizable(false)
            .collapsible(true)
            .title_bar(true)
            .scroll2([false, true])
            .movable(false)
            .fixed_size(self.window_size)
            .frame(self.panel_frame)
            .enabled(true);

        window.show(ctx, |ui| {
            self.frame_history_ui(ui);
        });
    }

    fn frame_history_ui(&mut self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.label("运行的 GPU 后端：");
            ui.colored_label(Color32::from_rgb(255, 125, 100), self.backend.to_str());
        });
        ui.separator();

        egui::Grid::new("my_grid")
            .num_columns(3)
            .spacing([10.0, 12.0])
            .striped(true)
            .show(ui, |ui| {
                self.frame_history.ui(ui);
            });
    }
}
