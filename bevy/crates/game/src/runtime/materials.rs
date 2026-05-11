use bevy::{
    prelude::*, reflect::TypePath, render::render_resource::AsBindGroup, shader::ShaderRef,
};

const CARD_BACKGROUND_MASK_SHADER_PATH: &str = "shaders/card_background_mask.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CardBackgroundMaskMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub background_texture: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    pub frame_texture: Handle<Image>,
    #[uniform(4)]
    pub inner_aperture: Vec4,
    pub alpha_mode: AlphaMode,
}

impl Material for CardBackgroundMaskMaterial {
    fn fragment_shader() -> ShaderRef {
        CARD_BACKGROUND_MASK_SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
