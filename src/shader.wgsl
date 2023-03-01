struct VertexInput {
    @location(0) color: vec3<f32>,
    @location(1) position: vec2<f32>,
};

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(1) color : vec4<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
  var out: VertexOutput;
  out.position = vec4<f32>(model.position, 0.5, 0.5);
  out.color = vec4<f32>(model.color, 1.0);
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
  return input.color;
}
