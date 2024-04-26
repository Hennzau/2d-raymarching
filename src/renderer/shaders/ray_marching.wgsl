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

@group(0)
@binding(1)
var<uniform> inverted_mvp: mat4x4<f32>;

@group(0)
@binding(2)
var<storage, read> world: array<u32, 1500>;

@fragment
fn fs_main(

    @builtin(position) in_frag_position: vec4<f32>,
    @location(0) in_vertex_color: vec4<f32>

) -> FragmentOutput {
    var result: FragmentOutput;

    let point_light = vec2<f32> (36.0 * 20.0, 10.0  * 20.0);

    let x = 2.0 * in_frag_position.x / 1600.0 - 1.0;
    let y = 1.0 - (2.0 * in_frag_position.y) / 900.0;

    let ray_nds = vec3<f32> (x, y, 1.0);
    let ray_clip = vec4<f32> (ray_nds.x, ray_nds.y, -1.0, 1.0);

    let ray_origin = (inverted_mvp * ray_clip).xy;
    let ray_direction = normalize (point_light - ray_origin);
    let tile_origin = ray_origin / 20;

    var t: f32 = 0.0;
    var ray = vec2<f32> (0);

    for (var i: i32 = 0; i < 100; i = i + 1) {
        let tile = vec2<u32> (u32((tile_origin + ray).x), u32((tile_origin + ray).y));

        // process world with tile

        if world[tile.x * 30 + tile.y] == 1 {
            break;
        }

        // increase parameters if the ray continue

        ray += ray_direction;

        t += 1.0;
    }

    result.out_frag_color = in_vertex_color * (1.0/t);

    return result;
}