use bevy::{asset::AssetMetaCheck, prelude::*};
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
        .run();
}
