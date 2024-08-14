struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
  var output: VertexOutput;

  output.position = vec4(input.position, 1.0);
  output.tex_coords = input.tex_coords;

  return output;
}

struct BaseShaderParameters {
  plane_id: i32,
  time: f32,
  output_resolution: vec2<u32>,
  texture_count: u32,
}

@group(0) @binding(0) var textures: binding_array<texture_2d<f32>, 16>;
@group(1) @binding(0) var<uniform> radius: f32;
@group(2) @binding(0) var sampler_: sampler;

var<push_constant> base_params: BaseShaderParameters;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {

  let color = textureSample(textures[0], sampler_, input.tex_coords);
  let aspect_ratio = f32(base_params.output_resolution.x) / f32(base_params.output_resolution.y);

  let coords = vec2<f32>(input.tex_coords.x*aspect_ratio, input.tex_coords.y);

  let center = vec2<f32>(0.5*aspect_ratio, 0.5);
  let distance = abs(coords - center);
  let limit = center - radius;

  if distance.x > limit.x && distance.y > limit.y {
    let distance = pow(pow(distance.x - limit.x, 2.0) + pow(distance.y - limit.y, 2.0), 0.5);

    if distance > radius {
      let opacity = 1.0 - (distance - radius) / 0.001;
      return vec4(color.rgb, opacity);
    }
  }

  return color;
}

