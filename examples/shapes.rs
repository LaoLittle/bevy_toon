use std::f32::consts::{PI, TAU};

use bevy::{
    core_pipeline::experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
    prelude::{
        shape::{Cube, Plane, Torus},
        *,
    },
    window::close_on_esc,
};

use bevy_toon::*;

#[bevy_main]
fn main() {
    App::new()
        //.insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((DefaultPlugins, ToonPlugin, TemporalAntiAliasPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (close_on_esc, wobble, orbit))
        .run();
}

#[derive(Component)]
struct Wobbles;

#[derive(Component)]
struct Orbits;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ToonMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        TemporalAntiAliasBundle::default(),
    ));

    commands
        .spawn(ToonBundle {
            mesh: meshes.add(Mesh::from(Cube { size: 1.0 })),
            material: materials.add(ToonMaterial {
                base_color: Color::rgb(0.1, 0.1, 0.9),
                smoothness: 0.02,
                outline: Some(OutlineSetting {
                    outline: OutlineVolume {
                        visible: true,
                        colour: Color::BLACK,
                        width: 5.0,
                    },
                    ..default()
                }),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .insert(Wobbles);

    // Add torus using the regular surface normals for outlining
    commands
        .spawn(ToonBundle {
            mesh: meshes.add(Mesh::from(Torus {
                radius: 0.3,
                ring_radius: 0.1,
                subdivisions_segments: 20,
                subdivisions_sides: 10,
            })),
            material: materials.add(ToonMaterial {
                base_color: Color::rgb(0.9, 0.1, 0.1),
                smoothness: 0.02,
                outline: Some(OutlineSetting {
                    outline: OutlineVolume {
                        visible: true,
                        colour: Color::PINK,
                        width: 4.0,
                    },
                    ..default()
                }),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 1.2, 2.0)
                .with_rotation(Quat::from_rotation_x(0.5 * PI)),
            ..default()
        })
        .insert(Orbits);

    // Add plane, light source, and camera
    commands.spawn(ToonBundle {
        mesh: meshes.add(Mesh::from(Plane {
            size: 5.0,
            subdivisions: 0,
        })),
        material: materials.add(ToonMaterial {
            base_color: Color::rgb(0.3, 0.5, 0.3),
            smoothness: 0.02,
            outline: None,
            ..default()
        }),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
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
}

fn wobble(mut query: Query<&mut Transform, With<Wobbles>>, timer: Res<Time>, mut t: Local<f32>) {
    let ta = *t;
    *t = (ta + 0.5 * timer.delta_seconds()) % TAU;
    let tb = *t;
    let i1 = tb.cos() - ta.cos();
    let i2 = ta.sin() - tb.sin();
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(
            TAU * 20.0 * i1 * timer.delta_seconds(),
        ));
        transform.rotate(Quat::from_rotation_y(
            TAU * 20.0 * i2 * timer.delta_seconds(),
        ));
    }
}

fn orbit(mut query: Query<&mut Transform, With<Orbits>>, timer: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(0.4 * timer.delta_seconds()),
        )
    }
}
