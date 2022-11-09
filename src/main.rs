mod enemy;
mod game;
mod tower;
mod builder;
mod ui;
mod world;

pub use enemy::*;
pub use game::*;
pub use tower::*;
pub use builder::*;
pub use ui::*;
pub use world::*;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    utils::FloatOrd,
};
use bevy_editor_pls::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_rapier3d::{
    prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
    render::{DebugRenderMode, RapierDebugRenderPlugin},
};
use bevy_scene_hook::HookPlugin;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "Bevy Tower Defense 0.1".to_string(),
            resizable: false,
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(RapierConfiguration {
            gravity: Vec3::ZERO,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        // Editor Plugin
        .add_plugin(EditorPlugin)
        // Mod Picking
        .add_plugins(DefaultPickingPlugins)
        // Physics
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin {
            mode: DebugRenderMode::COLLIDER_SHAPES,
            ..default()
        })
        // Scene Hooks
        .add_plugin(HookPlugin)
        // Our Plugins
        .add_plugins(DefaultGamePlugins)
        .add_plugins(DefaultWorldPlugins)
        .add_plugin(TowerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(BuilderPlugin)
        .add_plugin(UiPlugin)
        // Debug Systems
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}
