const corners_constants = array<vec4<f32>, 4>(
    vec4<f32>(1.0,-1.0,0.0,1.0),
    vec4<f32>(1.0,1.0,0.0,1.0),
    vec4<f32>(-1.0,-1.0,0.0,1.0),
    vec4<f32>(-1.0,1.0,0.0,1.0),
);
const margin: f32 = 0.2;

struct ColorWheel {
    color: vec4<f32>,
    resolution: vec2<f32>,
};

var<push_constant> pc: ColorWheel;

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32
) -> @builtin(position)  vec4<f32> {
    // https://github.com/gfx-rs/naga/issues/1910
    var corners = corners_constants;
    return corners[in_vertex_index];
}

fn color_square(uv: vec2<f32>) -> vec3<f32>{
    var lightness: vec3<f32> = vec3<f32>(1.0) * uv.y;
    var saturation: vec3<f32> = (1.0 - uv.x) * (lightness) + uv.x * pc.color.rgb;
    return saturation;
}

fn color_square_scaled(uv: vec2<f32>) -> vec4<f32>{
  var margin_other = margin - 1.0;
  var clip: f32 = step(margin, uv.x) * step(margin_other, -uv.x) * step(margin, uv.y) * step(margin_other, -uv.y);
  var uv_in_margin: vec2<f32> = (uv - margin)/ (1.0-(2.0*margin));
  return clip * vec4<f32>(color_square(uv_in_margin), clip);
}

fn compute_primary_color(angle: f32) -> f32 {
    let two_pi_by_three: f32 = acos(-0.5);
    var normalized_angle: f32 = atan2(cos(angle), sin(angle));
    var red: f32 = 1.0 - (abs(normalized_angle)/ two_pi_by_three);
    return clamp(red, 0.0, 1.0);
}

fn color_wheel(uv: vec2<f32>)-> vec4<f32> {
    let two_pi_by_three: f32 = acos(-0.5);
    let min_radius: f32 = sqrt(2.0) * (0.5 - margin);

    var radius: vec2<f32> = uv - 0.5;
    let angle: f32 = atan2(radius.y, radius.x);
    let len: f32 = length(radius);
    let clip: f32 = step(min_radius, len) * step(-0.5, -len);
    return clip * vec4<f32>(
        compute_primary_color(angle),
        compute_primary_color(angle-two_pi_by_three),
        compute_primary_color(angle+two_pi_by_three),
        clip
    );
}

@fragment
fn fs_main(@builtin(position) in: vec4<f32>) -> @location(0) vec4<f32> {
    var normal_in: vec2<f32> = in.xy / pc.resolution.xy;
    var down_right_in = (normal_in - vec2<f32>(0.73, 0.02)) * 4.0;
    return color_square_scaled(down_right_in) + color_wheel(down_right_in);
}
