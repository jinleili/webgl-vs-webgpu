use crate::util::vertex::PosNormalUv;
use truck_meshalgo::prelude::*;
use truck_modeling::*;

/// 圆柱体
#[allow(unused)]
pub(crate) fn create(height: f64, radius: f64) -> (PolygonMesh<usize, Vec<PosNormalUv>>, Vec<u32>) {
    let vertex = builder::vertex(Point3::new(0.0, radius, -height / 2.0));
    let circle = builder::rsweep(&vertex, Point3::origin(), Vector3::unit_z(), Rad(7.0));
    let face = builder::try_attach_plane(&[circle]).unwrap();
    let solid = builder::tsweep(&face, Vector3::new(0.0, 0.0, height));

    super::gen_mesh(solid, height.min(radius.min(1.)))
}
