let corners_constants = array<vec4<f32>, 4>(
    vec4<f32>(1.0,-1.0,0.0,1.0),
    vec4<f32>(1.0,1.0,0.0,1.0),
    vec4<f32>(-1.0,-1.0,0.0,1.0),
    vec4<f32>(-1.0,1.0,0.0,1.0),
);

struct ColorWheel {
    color: vec4<f32>;
    resolution: vec2<f32>;
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

fn color_square(uv: vec2<f32>) -> vec4<f32>{
    var lightness: vec3<f32> = vec3<f32>(1.0) * uv.y;
    var saturation: vec3<f32> = (1.0 - uv.x) * (lightness) + uv.x * pc.color.rgb;
    return vec4<f32>(saturation, 1.0);
}

[[stage(fragment)]]
fn fs_main([[builtin(position)]] in: vec4<f32>) -> [[location(0)]] vec4<f32> {
    var normal_in: vec2<f32> = in.xy / pc.resolution.xy;
    return color_square(normal_in);
}
