mod bullet;
mod game;
mod physics;
mod target;
mod tower;
mod camera;

pub use bullet::*;
pub use game::*;
pub use physics::*;
pub use target::*;
pub use tower::*;
pub use camera::*;

use bevy::{prelude::*, utils::FloatOrd};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::{
    prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
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
        // Inspector Plugin
        .add_plugin(WorldInspectorPlugin::new())
        // Physics
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        // Our Plugins
        .add_plugin(GamePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(TowerPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(BulletPlugin)
        // Level Systems
        .add_startup_system(spawn_level)
        .run();
}

fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    // set gravity
    rapier_config.gravity = Vec3::ZERO;

    // Spawn Ground
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Name::new("Ground"));

    // Spawn Tower
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(2.0, true),
            bullet_offset: Vec3::new(0.0, 0.2, 0.5),
        })
        .insert(Name::new("Tower"));

    // Spawn Main Light
    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));

    // Spawn Enemies (will spawn 3)
    for n in 1..4 {
        let x_pos = -2.0 * n as f32;

        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
                material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
                transform: Transform::from_xyz(x_pos, 0.2, 1.5),
                ..default()
            })
            .insert(Target { speed: 0.3 })
            .insert(Health { value: 3 })
            .insert_bundle(PhysicsBundle::moving_entity(Vec3::new(0.4, 0.4, 0.4)))
            .insert(Name::new("Target"));
    }
}
