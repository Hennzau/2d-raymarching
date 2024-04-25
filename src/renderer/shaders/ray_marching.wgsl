struct VertexOutput {
    @builtin(position) out_vertex_pos: vec4<f32>
}

struct FragmentOutput {
    @location(0) out_frag_color: vec4<f32>
}

@group(0)
@binding(0)
var<uniform> projection_matrix: mat4x4<f32>;

@group(0)
@binding(1)
var<uniform> view_matrix: mat4x4<f32>;

@group(0)
@binding(2)
var<uniform> model_matrix: mat4x4<f32>;

@vertex
fn vs_main(

    @location(0) in_vertex_position: vec2<f32>

) -> VertexOutput {
    var result: VertexOutput;

    result.out_vertex_pos = projection_matrix * view_matrix * model_matrix * vec4<f32> (in_vertex_position.x, in_vertex_position.y, 0.0, 1.0);

    return result;
}

@group(0)
@binding(3)
var<uniform> inverted_projection_matrix: mat4x4<f32>;

@group(0)
@binding(4)
var<uniform> inverted_view_matrix: mat4x4<f32>;

@group(0)
@binding(5)
var<uniform> surface_configuration: vec2<f32>;

@fragment
fn fs_main(

    @builtin(position) in_frag_position: vec4<f32>,

) -> FragmentOutput {
    var result: FragmentOutput;

    result.out_frag_color = vec4<f32> (1.0, 0.0, 0.0, 1.0);

    return result;
}