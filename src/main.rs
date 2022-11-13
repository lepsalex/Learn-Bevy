mod builder;
mod enemy;
mod game;
mod tower;
mod ui;
mod world;

pub use builder::*;
pub use enemy::*;
pub use game::*;
pub use tower::*;
pub use ui::*;
pub use world::*;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    utils::FloatOrd,
};
use bevy_inspector_egui::WorldInspectorPlugin;
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
        .add_plugins(DefaultPlugins.set(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "Bevy Tower Defense 0.1".to_string(),
            resizable: false,
            ..default()
        }))
        // Inspector Plugin
        .add_plugin(WorldInspectorPlugin::new())
        // Mod Picking
        .add_plugins(DefaultPickingPlugins)
        // Physics
        .add_plugin(
            RapierPhysicsPlugin::<NoUserData>::default().set(RapierConfiguration {
                gravity: Vec3::ZERO,
                ..default()
            }),
        )
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
