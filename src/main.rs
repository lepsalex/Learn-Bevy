mod camera;
mod game;
mod levels;
mod physics;
mod projectile;
mod target;
mod tower;

pub use camera::*;
pub use game::*;
pub use levels::*;
pub use physics::*;
pub use projectile::*;
pub use target::*;
pub use tower::*;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    utils::FloatOrd,
};
use bevy_editor_pls::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::{DebugRenderMode, RapierDebugRenderPlugin},
};
use bevy_scene_hook::HookPlugin;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hsl(42.0, 0.42, 0.61)))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "Learn Bevy 1.0".to_string(),
            resizable: false,
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
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
        .add_plugin(GamePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(TowerPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(ProjectilePlugin)
        // Level Systems
        .add_startup_system(levels::level_0::spawn_level)
        // Debug Systems
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}
