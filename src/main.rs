mod bullet;
mod game;
mod target;
mod tower;

pub use bullet::*;
pub use game::*;
pub use target::*;
pub use tower::*;

use bevy::{prelude::*, utils::FloatOrd};
use bevy_inspector_egui::WorldInspectorPlugin;

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
        // Our Plugins
        .add_plugin(GamePlugin)
        .add_plugin(TowerPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(BulletPlugin)
        // Level Systems
        .add_startup_system(spawn_level)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn Ground
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
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
            shooting_timer: Timer::from_seconds(1.0, true),
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
            .insert(Name::new("Target"));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Name::new("Main Camera"));
}
