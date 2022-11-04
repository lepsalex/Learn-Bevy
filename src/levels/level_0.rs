pub use crate::bullet::*;
pub use crate::camera::*;
pub use crate::game::*;
pub use crate::physics::*;
pub use crate::target::*;
pub use crate::tower::*;

use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_mod_picking::{Highlighting, PickableBundle};
use bevy_rapier3d::prelude::RapierConfiguration;
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

pub fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    game_assets: Res<GameAssets>,
) {
    // Set Gravity
    rapier_config.gravity = Vec3::ZERO;

    // Spawn Tower Base
    let default_collider_color = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into());
    let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9).into());
    let collider_mesh = meshes.add(shape::Capsule::default().into());

    // Spawn Level
    commands
        .spawn_bundle(HookedSceneBundle {
            scene: SceneBundle {
                scene: game_assets.level_0.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            hook: SceneHook::new(move |entity, cmds| {
                entity.get::<Name>().map(|name| {
                    /*
                    Attach required components for
                    tower base within the Blender scene

                    TODO: Consider refactoring this to a module?
                    */
                    if name.starts_with("base") {
                        cmds.insert(Name::new("Tower_Base"))
                            .insert(collider_mesh.clone())
                            .insert(Highlighting {
                                initial: default_collider_color.clone(),
                                hovered: Some(selected_collider_color.clone()),
                                pressed: Some(selected_collider_color.clone()),
                                selected: Some(selected_collider_color.clone()),
                            })
                            .insert(default_collider_color.clone())
                            .insert(NotShadowCaster)
                            .insert_bundle(PickableBundle::default());
                    }
                });
            }),
        })
        .insert(Name::new("Level"));

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
            .spawn_bundle(SpatialBundle {
                transform: Transform::from_xyz(x_pos, 0.4, 2.5),
                ..default()
            })
            .insert(Target { speed: 0.3 })
            .insert(Health { value: 3 })
            .insert_bundle(PhysicsBundle::moving_entity_sphere(0.4))
            .insert(Name::new("Target"))
            .with_children(|commands| {
                commands.spawn_bundle(SceneBundle {
                    scene: game_assets.target_scene.clone(),
                    ..default()
                });
            });
    }
}
