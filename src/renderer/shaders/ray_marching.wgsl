struct VertexOutput {
    @builtin(position) out_vertex_pos: vec4<f32>
}

struct FragmentOutput {
    @location(0) out_frag_color: vec4<f32>
}

@vertex
fn vs_main(

    @location(0) in_vertex_position: vec2<f32>

) -> VertexOutput {
    var result: VertexOutput;

    result.out_vertex_pos = vec4<f32> (in_vertex_position.x, in_vertex_position.y, 0.0, 1.0);

    return result;
}

@group(0)
@binding(0)
var<uniform> inverted_mvp: mat4x4<f32>;

@group(0)
@binding(1)
var<uniform> surface_configuration: vec2<f32>;

@group(0)
@binding(2)
var<uniform> point_light: vec2<f32>;

@group(0)
@binding(3)
var<storage, read> world: array<u32, 1500>;



@fragment
fn fs_main(

    @builtin(position) in_frag_position: vec4<f32>

) -> FragmentOutput {
    var result: FragmentOutput;

    let x = 2.0 * in_frag_position.x / surface_configuration.x - 1.0;
    let y = 1.0 - (2.0 * in_frag_position.y) / surface_configuration.y;

    let ray_nds = vec3<f32> (x, y, 1.0);
    let ray_clip = vec4<f32> (ray_nds.x, ray_nds.y, -1.0, 1.0);

    let ray_origin = (inverted_mvp * ray_clip).xy;
    let ray_direction = normalize (point_light - ray_origin);
    let light_distance = length (ray_origin - point_light);

    let tile_origin = vec2<i32> (i32(ray_origin.x / 20), i32(ray_origin.y / 20));

    if tile_origin.x >= 0 && tile_origin.x < 50 && tile_origin.y >= 0 && tile_origin.y < 30 {
        if world[tile_origin.x * 30 + tile_origin.y] == 0 {
            result.out_frag_color = vec4<f32> (1.0, 1.0, 0.0, 1.0);

            var final_luminosity = 0.0;

            var march = 0.0;
            var ray_target = ray_origin;
            for (var i: i32 = 0; i < 50; i = i + 1) {
                ray_target = ray_target + ray_direction * (march + 1);

                if length (ray_target - ray_origin) >= length(point_light - ray_origin) {
                    final_luminosity = 1.0;
                    break;
                }

                let ray_tile = vec2<i32> (i32(ray_target.x / 20), i32(ray_target.y / 20));

                if world[ray_tile.x * 30 + ray_tile.y] == 1 {
                    final_luminosity = 0.0;
                    break;
                }

                var k = array<f32, 4> (0.0, 0.0, 0.0, 0.0);

                let d = 20.0;

                if (ray_direction.x == 0.0 && ray_direction.y != 0) {
                    k[1] = (f32(ray_tile.y) * d + d - ray_target.y) / ray_direction.y;
                    k[3] = (f32(ray_tile.y) * d - ray_target.y) / ray_direction.y;
                } else if (ray_direction.x != 0.0 && ray_direction.y == 0) {
                    k[0] = (f32(ray_tile.x) * d - ray_target.x) / ray_direction.x;
                    k[2] = (f32(ray_tile.x) * d + d - ray_target.x) / ray_direction.x;
                } else {
                    k[0] = (f32(ray_tile.x) * d - ray_target.x) / ray_direction.x;
                    k[2] = (f32(ray_tile.x) * d + d - ray_target.x) / ray_direction.x;
                    k[1] = (f32(ray_tile.y) * d + d - ray_target.y) / ray_direction.y;
                    k[3] = (f32(ray_tile.y) * d - ray_target.y) / ray_direction.y;
                }

                var k_d = 0.0;
                for (var j: i32 = 0; j < 4; j = j + 1) {
                    if k[j] > 0.0 && (k_d == 0.0 || k_d > k[j]) {
                        k_d = k[j];
                    }
                }

                march = k_d;
            }

            result.out_frag_color = vec4<f32> (1.0, 1.0, 0.0, 1.0) * final_luminosity;
        } else {
            result.out_frag_color = vec4<f32> (1.0, 0.0, 0.0, 1.0);
        }
    } else {
        result.out_frag_color = vec4<f32> (0.0, 0.0, 0.0, 1.0);
    }

    return result;
}