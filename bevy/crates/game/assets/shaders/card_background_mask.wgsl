#import bevy_pbr::forward_io::VertexOutput

const CARD_LAYER_ROLE_BACKGROUND: f32 = 0.0;
const CARD_LAYER_ROLE_FRAME: f32 = 1.0;
const CARD_LAYER_ROLE_SAFE_AREA: f32 = 2.0;
const CARD_LAYER_ROLE_FOREGROUND: f32 = 3.0;
const CARD_LAYER_ROLE_TITLE: f32 = 4.0;

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var layer_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var layer_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var normal_map_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var normal_map_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var frame_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var frame_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(6) var<uniform> inner_aperture: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(7) var<uniform> vfx_layer: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(8) var<uniform> vfx_tilt: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(9) var<uniform> vfx_sweep: vec4<f32>;

fn decoded_normal(raw_normal: vec4<f32>) -> vec3<f32> {
    let tangent_normal = (raw_normal.xyz * 2.0) - vec3<f32>(1.0, 1.0, 1.0);
    return normalize(vec3<f32>(tangent_normal.x, -tangent_normal.y, tangent_normal.z));
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let role = vfx_layer.x;
    let normal_strength = max(vfx_layer.y, 0.0);
    let rim_strength = max(vfx_layer.z, 0.0);
    let plasma_strength = max(vfx_layer.w, 0.0);

    var layer_color = textureSample(layer_texture, layer_sampler, in.uv);

    let is_background = role == CARD_LAYER_ROLE_BACKGROUND;
    if (is_background) {
        let inside_x =
            step(inner_aperture.x, in.uv_b.x) * step(in.uv_b.x, inner_aperture.z);
        let inside_y =
            step(inner_aperture.y, in.uv_b.y) * step(in.uv_b.y, inner_aperture.w);
        let frame_hole_mask = (1.0 - textureSample(frame_texture, frame_sampler, in.uv_b).a)
            * inside_x
            * inside_y;
        layer_color.a *= frame_hole_mask;
    }

    let is_foreground = role == CARD_LAYER_ROLE_FOREGROUND;
    let is_title = role == CARD_LAYER_ROLE_TITLE;
    let is_frame = role == CARD_LAYER_ROLE_FRAME;

    if (is_foreground || is_title || is_frame) {
        let light_dir = normalize(vec3<f32>(vfx_tilt.x, vfx_tilt.y, 1.0));
        let view_dir = vec3<f32>(0.0, 0.0, 1.0);

        var normal = vec3<f32>(0.0, 0.0, 1.0);
        if (is_foreground || is_title) {
            normal = mix(
                normal,
                decoded_normal(textureSample(normal_map_texture, normal_map_sampler, in.uv)),
                normal_strength,
            );
        }

        let ndotl = clamp(dot(normalize(normal), light_dir), 0.0, 1.0);
        let lighting = mix(0.20, 1.0, pow(ndotl, 1.5));
        layer_color.rgb *= lighting;

        if (is_frame) {
            let rim_mask = textureSample(frame_texture, frame_sampler, in.uv).a;
            let rim = pow(1.0 - clamp(dot(normalize(vec3<f32>(vfx_tilt.x, -vfx_tilt.y, 1.0)), view_dir), 0.0, 1.0), 3.0);
            layer_color.rgb += rim_mask * rim_strength * vec3<f32>(0.15, 0.55, 1.0) * rim;
        }
    }

    let cycle_seconds = max(vfx_sweep.y, 0.001);
    let active_seconds = max(vfx_sweep.z, 0.001);
    let sweep_active_ratio = min(active_seconds / cycle_seconds, 1.0);
    let cycle_phase = fract(vfx_sweep.x / cycle_seconds);
    let is_plasma_phase = cycle_phase < sweep_active_ratio;
    let sweep_progress = (cycle_phase / sweep_active_ratio);
    let supports_plasma = is_foreground || is_title || is_frame;

    if (supports_plasma && is_plasma_phase) {
        let diagonal = in.uv.x + in.uv.y;
        let line_position = (sweep_progress * 1.4) - 0.2;
        let band_distance = abs(diagonal - line_position);
        let plasma_mask = 1.0 - smoothstep(0.0, 0.12, band_distance);
        layer_color.rgb = clamp(
            layer_color.rgb +
                (plasma_mask * plasma_strength * vec3<f32>(0.12, 0.90, 1.0) * (1.0 + (1.0 - rim_strength))),
            vec3<f32>(0.0, 0.0, 0.0),
            vec3<f32>(1.0, 1.0, 1.0),
        );
    }

    return layer_color;
}
