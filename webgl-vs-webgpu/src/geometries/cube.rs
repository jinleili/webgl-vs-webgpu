use crate::util::vertex::PosNormalUv;
use truck_meshalgo::prelude::*;
use truck_modeling::*;

/// 立方体
#[allow(unused)]
pub(crate) fn create(
    length: f64,
    width: f64,
    height: f64,
) -> (PolygonMesh<usize, Vec<PosNormalUv>>, Vec<u32>) {
    let half_len = length / 2.0;
    let half_w = width / 2.0;
    let half_h = height / 2.0;

    // 上左下右（逆时针）
    let v0 = builder::vertex(Point3::new(half_len, half_w, -half_h));
    let v1 = builder::vertex(Point3::new(-half_len, half_w, -half_h));
    let v2 = builder::vertex(Point3::new(-half_len, -half_w, -half_h));
    let v3 = builder::vertex(Point3::new(half_len, -half_w, -half_h));
    let wire: Wire = vec![
        builder::line(&v0, &v1),
        builder::line(&v1, &v2),
        builder::line(&v2, &v3),
        builder::line(&v3, &v0),
    ]
    .into();
    // 线框围成面
    let face = builder::try_attach_plane(&[wire]).unwrap();
    // 面扫成实体
    let solid = builder::tsweep(&face, Vector3::new(0.0, 0.0, height));

    super::gen_mesh(solid, length.min(width.min(height.min(20.0))))
}
