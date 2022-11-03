mod bullet;
mod camera;
mod game;
mod levels;
mod physics;
mod target;
mod tower;

pub use bullet::*;
pub use camera::*;
pub use game::*;
pub use levels::*;
pub use physics::*;
pub use target::*;
pub use tower::*;

use bevy::{prelude::*, utils::FloatOrd};
use bevy_editor_pls::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::{DebugRenderMode, RapierDebugRenderPlugin},
};

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "Learn Bevy 1.0".to_string(),
            resizable: false,
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
        // Our Plugins
        .add_plugin(GamePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(TowerPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(BulletPlugin)
        // Level Systems
        .add_startup_system(levels::level_0::spawn_level)
        .run();
}
