use std::f32::consts::FRAC_PI_2;

use bevy::{asset::AssetMetaCheck, prelude::*, render::camera::ScalingMode};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_kira_audio::AudioPlugin;

const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never) // Makes WASM happy
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "DDD".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AudioPlugin) // Kira audio
        // .add_plugins((
        //     RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
        //     RapierDebugRenderPlugin::default(),
        // ))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
        ..default()
    });

    // cube
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Cuboid::new(1., 1., 1.)),
    //     material: materials.add(Color::rgb_u8(124, 144, 255)),
    //     transform: Transform::from_xyz(0., 0.5, 0.),
    //     ..default()
    // });

    let transform = Transform {
        translation: Vec3::new(0., 0., 0.),
        rotation: Quat::from_rotation_x(-FRAC_PI_2),
        ..default()
    };

    let my_gltf = ass.load("LowPolychars3.glb#Scene0");
    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform,
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4., 8., 4.),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.5, 3., 4.).looking_at(Vec3::ZERO, Vec3::Y),
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(6.),
            ..default()
        }.into(),
        ..default()
    });
}
