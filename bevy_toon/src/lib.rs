use bevy::{asset::load_internal_asset, prelude::*};
use bevy_toon_outline::{AutoGenerateOutlineNormalsPlugin, OutlinePlugin};

mod material;
mod systems;
mod uniform;

use systems::*;

pub use bevy_toon_outline::{
    ComputedOutline, OutlineBundle, OutlineMode, OutlineStencil, OutlineVolume,
};
pub use material::{OutlineSetting, RimLightSetting, ToonMaterial};

use material::TOON_SHADER_HANDLE;

pub type ToonBundle = MaterialMeshBundle<ToonMaterial>;

pub struct ToonPlugin;

impl Plugin for ToonPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, TOON_SHADER_HANDLE, "toon.wgsl", Shader::from_wgsl);

        app.add_plugins((
            MaterialPlugin::<ToonMaterial>::default(),
            OutlinePlugin,
            AutoGenerateOutlineNormalsPlugin,
        ));

        app.add_systems(Update, (set_outline,));
    }
}
