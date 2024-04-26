struct VertexOutput {
    @builtin(position) out_vertex_pos: vec4<f32>,
    @location(0) out_vertex_color: vec4<f32>
}

struct FragmentOutput {
    @location(0) out_frag_color: vec4<f32>
}

@group(0)
@binding(0)
var<uniform> mvp: mat4x4<f32>;

@vertex
fn vs_main(

    @location(0) in_vertex_position: vec2<f32>,
    @location(1) in_vertex_color: vec4<f32>,

) -> VertexOutput {
    var result: VertexOutput;

    result.out_vertex_pos = mvp * vec4<f32> (in_vertex_position.x, in_vertex_position.y, 0.0, 1.0);
    result.out_vertex_color = in_vertex_color;

    return result;
}

@fragment
fn fs_main(

    @location(0) in_vertex_color: vec4<f32>

) -> FragmentOutput {
    var result: FragmentOutput;

    result.out_frag_color = in_vertex_color;

    return result;
}