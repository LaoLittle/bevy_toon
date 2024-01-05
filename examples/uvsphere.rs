use bevy::core_pipeline::experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin};
use bevy::prelude::*;
use bevy_toon::*;
use std::f32::consts::PI;

#[bevy_main]
fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, TemporalAntiAliasPlugin, ToonPlugin));

    app.add_systems(Startup, setup);

    app.run();
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mymat: ResMut<Assets<ToonMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0., 5.0),
            ..default()
        },
        TemporalAntiAliasBundle::default(),
    ));

    commands.spawn((DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10_000.,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(2.0, 2.0, 2.0),
            rotation: Quat::from_euler(EulerRot::XYZ, -PI / 4., PI / 6., 0.),
            ..default()
        },
        ..default()
    },));

    let mine = mymat.add(ToonMaterial {
        base_color: Color::TURQUOISE,
        smoothness: 0.05,
        outline: Some(OutlineSetting {
            outline: OutlineVolume {
                width: 5.0,
                visible: true,
                colour: Color::BLACK,
            },
            ..default()
        }),
        rim_light: Some(RimLightSetting {
            rim_color: Color::WHITE,
        }),
        ..default()
    });

    let ball = meshes.add(shape::UVSphere::default().into());

    commands.spawn(ToonBundle {
        material: mine,
        mesh: ball,
        ..default()
    });
}
