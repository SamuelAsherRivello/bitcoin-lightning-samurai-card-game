#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var screen_sampler: sampler;

struct DeckPromptBackdropBlur {
    texel_radius: f32,
}

@group(0) @binding(2) var<uniform> blur: DeckPromptBackdropBlur;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let dims = vec2<f32>(textureDimensions(screen_texture, 0));
    let texel = vec2<f32>(1.0 / dims.x, 1.0 / dims.y) * max(blur.texel_radius, 0.001);
    let uv = in.uv;

    // Lightweight 9-tap blur to soften full-scene detail behind modal overlays.
    var color = textureSample(screen_texture, screen_sampler, uv) * 0.20;
    color += textureSample(screen_texture, screen_sampler, uv + vec2<f32>( texel.x, 0.0)) * 0.12;
    color += textureSample(screen_texture, screen_sampler, uv + vec2<f32>(-texel.x, 0.0)) * 0.12;
    color += textureSample(screen_texture, screen_sampler, uv + vec2<f32>(0.0,  texel.y)) * 0.12;
    color += textureSample(screen_texture, screen_sampler, uv + vec2<f32>(0.0, -texel.y)) * 0.12;
    color += textureSample(screen_texture, screen_sampler, uv + vec2<f32>( texel.x,  texel.y)) * 0.08;
    color += textureSample(screen_texture, screen_sampler, uv + vec2<f32>(-texel.x,  texel.y)) * 0.08;
    color += textureSample(screen_texture, screen_sampler, uv + vec2<f32>( texel.x, -texel.y)) * 0.08;
    color += textureSample(screen_texture, screen_sampler, uv + vec2<f32>(-texel.x, -texel.y)) * 0.08;

    return color;
}
