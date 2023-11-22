use bevy::{prelude::*, render::render_resource::ShaderType};

#[derive(Clone, Default, ShaderType)]
pub struct ToonMaterialUniform {
    pub base_color: Color,
    pub smoothness: f32,

    pub rim_light_visible: u32,
    pub rim_color: Color,
}
