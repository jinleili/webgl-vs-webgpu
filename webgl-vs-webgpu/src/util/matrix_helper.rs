pub struct FullscreenFactor {
    pub sx: f32,
    pub sy: f32,
    pub translate_z: f32,
}

// 将[-1, 1]的矩形空间映射到刚好填充整个视口
#[allow(dead_code)]
pub fn perspective_fullscreen_mvp(viewport: glam::Vec2) -> (glam::Mat4, glam::Mat4) {
    let (p_matrix, vm_matrix, factor) = perspective_mvp(viewport);
    let scale_matrix = glam::Mat4::from_scale(glam::Vec3::new(factor.sx, factor.sy, 1.0));

    (p_matrix, vm_matrix * scale_matrix)
}

pub fn perspective_mvp(viewport: glam::Vec2) -> (glam::Mat4, glam::Mat4, FullscreenFactor) {
    // 视场角越大，场景中实体的形变越严重
    let fovy: f32 = 45.0_f32.to_radians();
    let p_matrix = glam::Mat4::perspective_rh(fovy, viewport.x / viewport.y, 0.1, 1000.0);
    let factor = fullscreen_factor(viewport, fovy);
    // let vm_matrix = glam::Mat4::from_translation(glam::vec3(0.0, 0.0, factor.translate_z));
    let vm_matrix = glam::Mat4::IDENTITY;

    (p_matrix, vm_matrix, factor)
}

pub fn fullscreen_factor(viewport: glam::Vec2, fovy: f32) -> FullscreenFactor {
    // 缩放到贴合屏幕
    //
    // 移动近裁剪平面,屏幕上的投影并不会缩放,
    // 因为虽然模型对象在裁剪平面上看起来投影随之缩放,但裁剪平面本身也在随之缩放
    // 相当于是 裁剪平面与其上的投影在整体缩放, 而裁剪平面始终是等于屏幕空间平面的, 所以映射到屏幕上就是没有缩放
    // 满屏效果: 默认 camera 在原点，利用 fovy 计算 tan (近裁剪平面 x | y 与 camera 原点的距离之比) 得出 z 轴平移距离
    // 屏幕 h > w 时，才需要计算 ratio, w > h 时， ration = 1
    let mut sx: f32 = 1.0;
    let mut sy = 1.0;

    let ratio = if viewport.y > viewport.x {
        let ratio = viewport.y / viewport.x;
        sy = ratio;
        ratio
    } else {
        sx = viewport.x / viewport.y;
        1.0
    };
    // 右手坐标系，z 轴朝屏幕外，所以是负数
    let translate_z = -ratio / (fovy / 2.0).tan();

    FullscreenFactor {
        sx,
        sy,
        translate_z,
    }
}

pub fn ortho_mvp(viewport_size: glam::Vec2) -> (glam::Mat4, glam::Mat4) {
    let fovy: f32 = 45.0f32.to_radians();
    let factor = fullscreen_factor(viewport_size, fovy);
    let p_matrix = glam::Mat4::orthographic_rh(
        -1.0 * factor.sx,
        1.0 * factor.sx,
        -1.0 * factor.sy,
        1.0 * factor.sy,
        -100.0,
        100.0,
    );
    (p_matrix, glam::Mat4::IDENTITY)
}
