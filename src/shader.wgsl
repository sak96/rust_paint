struct ColorWheel {
    color: vec3<f32>;
    enabled: f32;
};

var<push_constant> pc: ColorWheel;

struct VertexInput {
    [[location(0)]] color: vec3<f32>;
    [[location(1)]] position: vec2<f32>;
    [[location(2)]] radius: f32;
};

struct VertexOutput {
  [[builtin(position)]] position: vec4<f32>;
  [[location(1)]] color : vec4<f32>;
  [[location(2)]] radius: f32;
};

[[stage(vertex)]]
fn vs_main(model: VertexInput) -> VertexOutput {
  var out: VertexOutput;
  out.position = vec4<f32>(model.position, 0.5, 0.5);
  out.color = vec4<f32>(model.color, 1.0);
    return out;
}

[[stage(fragment)]]
fn fs_main(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    if ( pc.enabled > 0.0 ) {
      return vec4<f32>(pc.color, 0.0);
    } else {
      return  input.color;
    }
}
