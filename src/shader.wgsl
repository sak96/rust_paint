struct VertexInput {
    [[location(0)]] position: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main(model: VertexInput) -> [[builtin(position)]] vec4<f32> {
    return vec4<f32>(model.position, 0.5, 0.5);
}

[[stage(fragment)]]
fn fs_main() -> [[location(0)]] vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
