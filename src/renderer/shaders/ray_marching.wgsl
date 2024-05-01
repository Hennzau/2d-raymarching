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

fn ray_check (ray_origin: vec2<f32>, ray_target: vec2<f32>) -> bool {
    let tile_size = 20.0;
    let ray_direction = normalize (ray_target - ray_origin);

    var march = 0.0;
    var ray = ray_origin;

    for (var i: i32 = 0; i < 50; i = i + 1) {
        ray = ray + ray_direction * march;

        if length (ray - ray_origin) >= length(ray_target - ray_origin) {
            return true;
        }

        if ray.x < 0.0 || ray.x >= 1000.0 || ray.y < 0.0 || ray.y >= 600.0 {
            return false;
        }

        let ray_tile = vec2<i32> (i32(ray.x / tile_size), i32(ray.y / tile_size));

        if ray_tile.x < 0 || ray_tile.x >= 50 || ray_tile.y < 0 || ray_tile.y >= 30 {
            return false;
        }

        if world[ray_tile.x * 30 + ray_tile.y] == 1 {
            return false;
        }

        var k = array<f32, 4> (0.0, 0.0, 0.0, 0.0);

        if ray_direction.y != 0 {
            k[1] = (f32(ray_tile.y) * tile_size + tile_size - ray.y) / ray_direction.y;
            k[3] = (f32(ray_tile.y) * tile_size - 1.0 - ray.y) / ray_direction.y;
        }

        if ray_direction.x != 0.0 {
            k[0] = (f32(ray_tile.x) * tile_size - 1.0 - ray.x) / ray_direction.x;
            k[2] = (f32(ray_tile.x) * tile_size + tile_size - ray.x) / ray_direction.x;
        }

        march = 0.0;

        for (var j: i32 = 0; j < 4; j = j + 1) {
            if k[j] > 0.0 && (march == 0.0 || march > k[j]) {
                march = k[j];
            }
        }
    }

    return true;
}

fn ray_hit (ray_origin: vec2<f32>, ray_direction: vec2<f32>) -> vec2<f32> {

}

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

    if ray_origin.x < 0.0 || ray_origin.x >= 1000.0 || ray_origin.y < 0.0 || ray_origin.y >= 600.0 {
        result.out_frag_color = vec4<f32> (0.0, 0.0, 0.0, 1.0);

        return result;
    }

    let tile_origin = vec2<i32> (i32(ray_origin.x / 20), i32(ray_origin.y / 20));

    if tile_origin.x < 0 || tile_origin.x >= 50 || tile_origin.y < 0 || tile_origin.y >= 30 {
        result.out_frag_color = vec4<f32> (0.0, 0.0, 0.0, 1.0);

        return result;
    }

    if world[tile_origin.x * 30 + tile_origin.y] == 1 {
        result.out_frag_color = vec4<f32> (1.0, 0.0, 0.0, 1.0);

        return result;
    }

    var final_luminosity = 0.0;

    if ray_check (ray_origin, point_light) {
        final_luminosity = 1.0;
    }

    result.out_frag_color = vec4<f32> (1.0, 1.0, 0.0, 1.0) * final_luminosity;

    return result;
}