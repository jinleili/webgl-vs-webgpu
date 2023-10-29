use crate::util::vertex::PosNormalUv;
use truck_meshalgo::prelude::*;
use truck_modeling::*;

mod sphere;
pub use sphere::Sphere;
pub mod cube;
pub mod cylinder;
pub mod bottle;

/// 生成网格顶点与索引
pub(crate) fn gen_mesh(solid: Solid, tol: f64) -> (PolygonMesh<usize, Vec<PosNormalUv>>, Vec<u32>) {
    let mesh = solid.triangulation(tol).to_polygon();

    // 顶点数据与索引
    expands_mesh(mesh)
}

pub(crate) fn expands_mesh(mesh: PolygonMesh) -> (PolygonMesh<usize, Vec<PosNormalUv>>, Vec<u32>) {
    let expanded = mesh.expands(|attr| PosNormalUv {
        pos: attr.position.cast().unwrap().into(),
        uv: attr
            .uv_coord
            .and_then(|v| Some(v.cast()?.into()))
            .unwrap_or([0.0, 0.0]),
        normal: attr
            .normal
            .and_then(|v| Some(v.cast()?.into()))
            .unwrap_or([0.0, 0.0, 0.0]),
    });
    let indices = expanded
        .faces()
        .triangle_iter()
        .flatten()
        .map(|x| x as u32)
        .collect::<Vec<_>>();
    (expanded, indices)
}
