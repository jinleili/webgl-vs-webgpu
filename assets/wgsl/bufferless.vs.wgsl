struct VertexOutput {
    @location(0) uv: vec2f,
    @builtin(position) position: vec4f,
};

@vertex
fn vs_main(@builtin(vertex_index) vertexIndex: u32) -> VertexOutput {
    let uv: vec2f = vec2f(f32((vertexIndex << 1u) & 2u), f32(vertexIndex & 2u));
    var out: VertexOutput;
    // Keep z slightly larger than 0, so that the egui layer is always on top.
    out.position = vec4f(uv * 2.0 - 1.0, 0.1, 1.0);
    // invert uv.y
    out.uv = vec2f(uv.x, (uv.y - 1.0) *  (-1.0));
    return out;
}