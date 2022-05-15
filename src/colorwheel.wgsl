let corners_constants = array<vec4<f32>, 4>(
    vec4<f32>(1.0,-1.0,0.0,1.0),
    vec4<f32>(1.0,1.0,0.0,1.0),
    vec4<f32>(-1.0,-1.0,0.0,1.0),
    vec4<f32>(-1.0,1.0,0.0,1.0),
);

struct ColorWheel {
    color: vec3<f32>;
    enabled: f32;
};

var<push_constant> pc: ColorWheel;

[[stage(vertex)]]
fn vs_main(
    [[builtin(vertex_index)]] in_vertex_index: u32
) -> [[builtin(position)]]  vec4<f32> {
    // https://github.com/gfx-rs/naga/issues/1910
    var corners = corners_constants;
    return corners[in_vertex_index];
}

[[stage(fragment)]]
fn fs_main([[builtin(position)]] in: vec4<f32>) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}
