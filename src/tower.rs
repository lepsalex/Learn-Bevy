use crate::{
    assets::GameAssets,
    common::{Lifetime, Target},
    physics::PhysicsBundle,
    projectile::Projectile,
    *,
};

use bevy::prelude::*;
use bevy_mod_picking::Selection;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
        .register_type::<TowerType>()
            .add_system(build_tower)
            .add_system(tower_shooting);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

#[derive(Reflect, Component, Clone, Copy, Default)]
#[reflect(Component)]
pub enum TowerType {
    #[default]
    Cannon,
    Catapult,
    Blaster,
}

fn build_tower(
    mut commands: Commands,
    selection: Query<(Entity, &Selection, &Transform)>,
    keyboard: Res<Input<KeyCode>>,
    assets: Res<GameAssets>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for (entity, selection, transform) in &selection {
            if selection.selected() {
                commands
                    .entity(entity)
                    .remove_bundle::<MarkedBuildLocationBundle>();
                spawn_tower(
                    &mut commands,
                    TowerType::Blaster,
                    &assets,
                    Vec3 {
                        x: transform.translation.x,
                        y: transform.translation.y + 0.9,
                        z: transform.translation.z,
                    },
                );
            }
        }
    }
}

fn spawn_tower(
    commands: &mut Commands,
    tower_type: TowerType,
    assets: &GameAssets,
    position: Vec3,
) -> Entity {
    let mut common_cmds = commands.spawn_bundle(SpatialBundle::from_transform(
        Transform::from_translation(position),
    ));

    match tower_type {
        TowerType::Cannon => common_cmds
            .insert(Name::new("Tower (Cannon)"))
            .insert(TowerType::Cannon)
            .insert(Tower {
                shooting_timer: Timer::from_seconds(0.5, true),
                bullet_offset: Vec3::new(0.0, 0.6, 0.0),
            })
            .with_children(|commands| {
                commands.spawn_bundle(SceneBundle {
                    scene: assets.tower_cannon_scene.clone(),
                    transform: Transform::from_xyz(0.0, -0.8, 0.0),
                    ..default()
                });
            })
            .id(),
        TowerType::Catapult => common_cmds
            .insert(Name::new("Tower (Catapult)"))
            .insert(TowerType::Catapult)
            .insert(Tower {
                shooting_timer: Timer::from_seconds(0.5, true),
                bullet_offset: Vec3::new(0.0, 0.6, 0.0),
            })
            .with_children(|commands| {
                commands.spawn_bundle(SceneBundle {
                    scene: assets.tower_catapult_scene.clone(),
                    transform: Transform::from_xyz(0.0, -0.8, 0.0),
                    ..default()
                });
            })
            .id(),
        TowerType::Blaster => common_cmds
            .insert(Name::new("Tower (Blaster)"))
            .insert(TowerType::Blaster)
            .insert(Tower {
                shooting_timer: Timer::from_seconds(0.5, true),
                bullet_offset: Vec3::new(0.0, 0.6, 0.0),
            })
            .with_children(|commands| {
                commands.spawn_bundle(SceneBundle {
                    scene: assets.tower_blaster_scene.clone(),
                    transform: Transform::from_xyz(0.0, -0.8, 0.0),
                    ..default()
                });
            })
            .id(),
    }
}

fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    game_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for (tower_entity, mut tower, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());

        if tower.shooting_timer.just_finished() {
            let bullet_spawn = transform.translation() + tower.bullet_offset;

            let bullet_direction = targets
                .iter()
                // order targets by distance, closest first
                .min_by_key(|target_transform| {
                    FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                })
                // convert closest target location to direction vector
                .map(|closest_target| closest_target.translation() - bullet_spawn);

            if let Some(bullet_direction) = bullet_direction {
                commands.entity(tower_entity).with_children(|commands| {
                    commands
                        .spawn_bundle(SpatialBundle {
                            transform: Transform::from_translation(tower.bullet_offset),
                            ..default()
                        })
                        .insert(Name::new("Bullet"))
                        .insert(Lifetime {
                            timer: Timer::from_seconds(10.0, false),
                        })
                        .insert(Projectile {
                            direction: bullet_direction,
                            speed: 2.5,
                            damage: 1,
                        })
                        .insert_bundle(PhysicsBundle::moving_entity_cube(Vec3::new(0.2, 0.2, 0.)))
                        .with_children(|commands| {
                            commands.spawn_bundle(SceneBundle {
                                scene: game_assets.cannon_ball_scene.clone(),
                                ..default()
                            });
                        });
                });
            }
        }
    }
}
