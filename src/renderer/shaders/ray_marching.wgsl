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
    let light_distance = length (ray_origin - point_light);

    let tile_origin = vec2<i32> (i32(ray_origin.x / 20), i32(ray_origin.y / 20));

    if tile_origin.x >= 0 && tile_origin.x < 50 && tile_origin.y >= 0 && tile_origin.y < 30 {
        if world[tile_origin.x * 30 + tile_origin.y] == 0 {
            result.out_frag_color = vec4<f32> (1.0, 1.0, 0.0, 1.0);

            var final_luminosity = 0.0;

            for (var i: i32 = 0; i < 30; i = i + 1) {
                let ray_direction = vec2<f32> (cos(2.0 * 3.141502 * f32(i) / 30.0f), sin(2.0 * 3.141502 * f32(i) / 30.0f));

                var distance = 0.0;
                var obstacle = false;
                var ray_target = ray_origin + ray_direction;

                for (var j: i32 = 0; j < 50; j = j + 1) {
                    ray_target = ray_origin + ray_direction * f32(j) * 20;

                    let tile_target = vec2<i32> (i32(ray_target.x / 20), i32(ray_target.y / 20));
                    if tile_target.x >= 0 && tile_target.x < 50 && tile_target.y >= 0 && tile_target.y < 30 {
                        distance = length (ray_target - ray_origin);

                        if world[tile_target.x * 30 + tile_target.y] == 1 {
                            ray_target = ray_origin + ray_direction * f32(j - 1) * 10;
                            obstacle = true;

                            break;
                        }
                    } else {
                        break;
                    }
                }

                if obstacle {
                    let ray_origin_2 = ray_target;
                    let ray_direction = normalize (point_light - ray_origin_2);

                    var distance_2 = 0.0;

                    for (var j: i32 = 0; j < 50; j = j + 1) {
                        ray_target = ray_origin_2 + ray_direction * f32(j) * 20;

                        let tile_target = vec2<i32> (i32(ray_target.x / 20), i32(ray_target.y / 20));
                        if tile_target.x >= 0 && tile_target.x < 50 && tile_target.y >= 0 && tile_target.y < 30 {
                            distance_2 = length(point_light - ray_origin);

                            if length (ray_target - ray_origin_2) >= length(point_light - ray_origin_2) {
                                obstacle = false;
                                break;
                            }

                            if world[tile_target.x * 30 + tile_target.y] == 1 {
                                obstacle = true;

                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    if obstacle == false {
                        let t_1 = -1.0 / 700.0 * distance + 1.0;
                        let t_2 = -1.0 / 700.0 * distance_2 + 1.0;

                        final_luminosity += (t_1 * t_2) / 50.0f;
                    }
                }
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