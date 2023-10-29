mod bottle_model;
mod thousands_entity;
use app_surface::AppSurface;
pub use thousands_entity::ThousandsEntity;

pub trait CompScenario {
    fn draw_by_pass<'a, 'b: 'a>(&'b mut self, app: &AppSurface, rpass: &mut wgpu::RenderPass<'b>);
}

/// 管线数量
/// 管线本身是可重用的，为了测试，用最差的做法
const PSO_COUNT: usize = 1000;
