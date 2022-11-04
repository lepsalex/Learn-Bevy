mod game;
mod physics;
mod projectile;
mod target;
mod tower;
mod world;

pub use game::*;
pub use level::*;
pub use physics::*;
pub use projectile::*;
pub use target::*;
pub use tower::*;
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
        .insert_resource(ClearColor(Color::hsl(42.0, 0.42, 0.61)))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "Learn Bevy 1.0".to_string(),
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
        .add_plugin(GamePlugin)
        .add_plugins(DefaultWorldPlugins)
        .add_plugin(TowerPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(ProjectilePlugin)
        // Spawn Level on Start
        .add_startup_system(spawn_level)
        // Debug Systems
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}

fn spawn_level(mut commands: Commands, game_assets: Res<GameAssets>) {
    // Spawn Enemies (will spawn 3)
    for n in 1..4 {
        let x_pos = -3.0 * n as f32;

        commands
            .spawn_bundle(SpatialBundle {
                transform: Transform::from_xyz(x_pos, 0.5, 0.5),
                ..default()
            })
            .insert(Target { speed: 0.6 })
            .insert(Health { value: 3 })
            .insert_bundle(PhysicsBundle::moving_entity_sphere(0.6))
            .insert(Name::new("Target"))
            .with_children(|commands| {
                commands.spawn_bundle(SceneBundle {
                    scene: game_assets.ufo_red_scene.clone(),
                    ..default()
                });
            });
    }
}
