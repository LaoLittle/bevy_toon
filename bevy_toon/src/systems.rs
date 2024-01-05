use bevy::prelude::*;

use crate::ToonMaterial;

pub fn set_outline(
    mut commands: Commands,
    query: Query<(Entity, &Handle<ToonMaterial>), Changed<Handle<ToonMaterial>>>,
    materials: Res<Assets<ToonMaterial>>,
) {
    for (entity, m) in &query {
        let m = materials.get(m);

        let Some(material) = m else {
            continue;
        };

        let Some(outline) = material.outline.clone() else {
            continue;
        };

        let Some(mut ec) = commands.get_entity(entity) else {
            continue;
        };

        ec.insert(outline.into_bundle());
    }
}
