#import bevy_pbr::forward_io::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var background_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var background_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var frame_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var frame_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> inner_aperture: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let background = textureSample(background_texture, background_sampler, in.uv);
    let frame = textureSample(frame_texture, frame_sampler, in.uv_b);
    let inside_x = step(inner_aperture.x, in.uv_b.x) * step(in.uv_b.x, inner_aperture.z);
    let inside_y = step(inner_aperture.y, in.uv_b.y) * step(in.uv_b.y, inner_aperture.w);
    let frame_hole_mask = (1.0 - frame.a) * inside_x * inside_y;

    return vec4<f32>(background.rgb, background.a * frame_hole_mask);
}
