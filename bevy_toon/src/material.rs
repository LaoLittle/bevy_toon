use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef};
use bevy_toon_outline::{
    ComputedOutline, OutlineBundle, OutlineMode, OutlineStencil, OutlineVolume,
};

use crate::uniform::ToonMaterialUniform;

pub const TOON_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(231420938561324824812043776083275844465);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
#[uniform(0, ToonMaterialUniform)]
pub struct ToonMaterial {
    pub base_color: Color,
    pub smoothness: f32,
    pub outline: Option<OutlineSetting>,
    pub rim_light: Option<RimLightSetting>,
}

#[derive(Debug, Clone, Default)]
pub struct OutlineSetting {
    pub outline: OutlineVolume,
    pub stencil: OutlineStencil,
    pub mode: OutlineMode,
    pub computed: ComputedOutline,
}

impl OutlineSetting {
    pub fn into_bundle(self) -> OutlineBundle {
        let Self {
            outline,
            stencil,
            mode,
            computed,
        } = self;

        OutlineBundle {
            outline,
            stencil,
            mode,
            computed,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct RimLightSetting {
    pub rim_color: Color,
}

impl AsBindGroupShaderType<ToonMaterialUniform> for ToonMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<Image>) -> ToonMaterialUniform {
        let mut uniform = ToonMaterialUniform {
            base_color: self.base_color,
            smoothness: self.smoothness,
            rim_light_visible: 0,
            rim_color: Color::NONE,
        };

        if let Some(RimLightSetting { rim_color }) = self.rim_light {
            uniform.rim_light_visible = 1;
            uniform.rim_color = rim_color;
        }

        uniform
    }
}

impl Material for ToonMaterial {
    fn fragment_shader() -> ShaderRef {
        TOON_SHADER_HANDLE.into()
    }
}
