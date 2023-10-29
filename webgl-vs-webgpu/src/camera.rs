use glam::Vec3Swizzles;
use winit::dpi::PhysicalPosition;

#[allow(unused)]
pub struct Camera {
    pub position: glam::Vec3,
    center: glam::Vec2,
    // xy 平面的旋转半径
    xy_radius: f32,
    // 上次旋转结束时的弧度
    last_end_radians: f32,
    // 旋转起始弧度
    start_radians: Option<f32>,
    /// YPR 欧拉角（Euler angles）
    /// Z 为 up 轴，X 为 right 轴，Y 为 forward 轴
    /// 偏航，绕着Z轴旋转
    yaw: f32,
    /// 俯仰，绕着Y轴旋转
    pitch: f32,
}

#[allow(unused)]
impl Camera {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let center = glam::Vec2::new(screen_width / 2., screen_height / 2.);
        let position = glam::Vec3::new(3.2, -1.0, 3.);
        let xy_radius = (glam::Vec2::new(0., 0.) - position.xy()).length();
        let mut instance = Self {
            position,
            center,
            xy_radius,
            last_end_radians: 0.0,
            start_radians: None,
            yaw: 0.0f32.to_radians(),
            pitch: 0.0,
        };
        instance.update_last_end_radians();
        instance
    }

    pub fn rotate(&mut self, position: PhysicalPosition<f64>) {
        let pos = (glam::Vec2::new(position.x as f32, position.y as f32) - self.center).normalize();
        let current_radians = glam::Vec2::new(1., 0.).angle_between(pos);
        // alt/option 键按下时，初始化旋转
        // 以随后的第一个 pos 来计算初始夹角;
        // 用此初始夹角来计算相机旋转量
        // alt/option 键松开时，结束旋转, 将本次旋转总量累计到 last_end_radians
        if let Some(start_radians) = self.start_radians {
            let delta = current_radians - start_radians;
            if delta.abs() < 0.000001 {
                return;
            }
            let total_radians = delta + self.last_end_radians;
            let (sin_dif, cos_dif) = total_radians.sin_cos();
            self.position.x = cos_dif * self.xy_radius;
            self.position.y = sin_dif * self.xy_radius;
        } else {
            self.start_radians = Some(current_radians);
        }
    }

    pub fn rotate_end(&mut self) {
        self.start_radians = None;
        self.update_last_end_radians();
    }

    pub fn calc_matrix(&self) -> glam::Mat4 {
        // 相机朝向
        let dir = (glam::Vec3::new(0., 0.0, 0.) - self.position).normalize();
        glam::Mat4::look_to_rh(self.position, dir, glam::Vec3::Z)
    }

    /// 上次旋转结束时的弧度
    fn update_last_end_radians(&mut self) {
        self.last_end_radians = glam::Vec2::new(1., 0.).angle_between(self.position.xy());
    }
}
