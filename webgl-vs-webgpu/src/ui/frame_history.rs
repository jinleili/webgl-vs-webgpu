use egui::util::History;

pub struct FrameHistory {
    // 上一帧开始到当前帧开始的时间间隔历史
    frame_times: History<f32>,
    // 一帧内的 cpu 耗时历史
    cpu_usage_times: History<f32>,
    // UI 刷新计数，用于降低 UI 上数值的刷新率（不影响准确性）
    refresh_count: u64,
    // 帧率
    frame_rate: f32,
    // 单帧 cpu 耗时
    cpu_usage: f32,
}

impl Default for FrameHistory {
    fn default() -> Self {
        // 最大时间步长为 1 秒
        let max_age: f32 = 1.0;
        Self {
            frame_times: History::new(0..200, max_age),
            cpu_usage_times: History::new(0..200, max_age),
            refresh_count: 0,
            frame_rate: 0.,
            cpu_usage: 0.,
        }
    }
}

#[allow(unused)]
impl FrameHistory {
    pub fn on_new_frame(&mut self, now: f64, frame_time: f32, cpu_usage_time: f32) {
        self.frame_times.add(now, frame_time);
        self.cpu_usage_times.add(now, cpu_usage_time);
        self.refresh_count += 1;
        if self.refresh_count % 20 == 0 {
            // 重新计算帧数据
            self.frame_rate = 1.0 / self.frame_times.average().unwrap_or_default();
            self.cpu_usage = self.cpu_usage_times.average().unwrap_or_default() * 1000.
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("CPU 耗时:");
        ui.label(format!("{:.1}", self.cpu_usage));
        ui.label("ms / 帧");
        ui.end_row();

        ui.label("实时帧率:");
        ui.label(format!("{:.1}", self.frame_rate));
        ui.label("帧 / 秒");
        ui.end_row();
    }
}
