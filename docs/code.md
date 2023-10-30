# 代码说明

页面右上角的 github 图标可跳转到代码库

## 使用的同一份代码编译为 WebGL 或 WebGPU 目标

**wgpu** (dawn 也是) 实现的 WebGPU 接口支持通过编译参数设置来运行时要使用的图形接口后端，但在使用 WebGL 时功能特性会受到限制。这与 WebGL 库对接 WebGPU 接口有所不同：因为 WebGPU 接口是基于管线的，由管线映可直接射回状态机。然而，基于状态机的接口映射到管线时，不仅功能受限，还会极大地降低管线的复用性，从而导致性能损耗。

```sh
# 构建 webgpu 包并运行
cargo run-wasm
# 构建 webgl 包并运行
cargo run-wasm --features=webgl
```

## 帧时间统计

帧时间统计由是前 200 帧计算的平均值：

```rust
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

impl FrameHistory {
    pub fn on_new_frame(&mut self, now: f64, frame_time: f32, cpu_usage_time: f32) {
        self.frame_times.add(now, frame_time);
        self.cpu_usage_times.add(now, cpu_usage_time);
        self.refresh_count += 1;
        if self.refresh_count % 20 == 0 {
            // 每 20 帧重新计算一次待显示的帧数据
            self.frame_rate = 1.0 / self.frame_times.average().unwrap_or_default();
            self.cpu_usage = self.cpu_usage_times.average().unwrap_or_default() * 1000.
        }
    }
}
```

## 测试场景

每帧 1000 个 draw call + 1000 次 buffer 修改：

```rust
/// 管线数量
/// 管线本身是可重用的，为了测试，用最差的做法
const PSO_COUNT: usize = 1000;

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
```
