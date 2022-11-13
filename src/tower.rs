use crate::{
    assets::GameAssets,
    common::{Lifetime, Target},
    navigation::NavAgent,
    physics::PhysicsBundle,
    projectile::Projectile,
    *,
};

use bevy::prelude::*;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .register_type::<TowerType>()
            .add_system(tower_shooting);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub projectile_offset: Vec3,
    pub range: f32,
}

#[derive(Reflect, Component, Clone, Copy, Default)]
#[reflect(Component)]
pub enum TowerType {
    #[default]
    Cannon,
    Catapult,
    Blaster,
}

pub fn spawn_tower(
    commands: &mut Commands,
    tower_type: TowerType,
    assets: &GameAssets,
    position: Vec3,
) -> Entity {
    match tower_type {
        TowerType::Cannon => commands.spawn((
            SceneBundle {
                scene: assets.tower_cannon_scene.clone(),
                transform: Transform::from_translation(position),
                ..default()
            },
            Name::new("Tower (Cannon)"),
            TowerType::Cannon,
            Tower {
                shooting_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                projectile_offset: Vec3::new(0.0, 0.6, 0.0),
                range: 3.0,
            },
        )),
        TowerType::Catapult => commands.spawn((
            SceneBundle {
                scene: assets.tower_catapult_scene.clone(),
                transform: Transform::from_translation(position),
                ..default()
            },
            Name::new("Tower (Catapult)"),
            TowerType::Catapult,
            Tower {
                shooting_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                projectile_offset: Vec3::new(0.0, 0.6, 0.0),
                range: 3.0,
            },
        )),
        TowerType::Blaster => commands.spawn((
            SceneBundle {
                scene: assets.tower_blaster_scene.clone(),
                transform: Transform::from_translation(position),
                ..default()
            },
            Name::new("Tower (Blaster)"),
            TowerType::Blaster,
            Tower {
                shooting_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                projectile_offset: Vec3::new(0.0, 0.6, 0.0),
                range: 3.0,
            },
        )),
    }
    .id()
}

fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<(&GlobalTransform, &NavAgent), With<Target>>,
    game_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for (tower_entity, mut tower, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());

        if tower.shooting_timer.just_finished() {
            let projectile_speed = 2.5; // move this to component
            let projectile_spawn = transform.translation() + tower.projectile_offset;

            let projectile_direction = targets
                .iter()
                // filter out targets that are out of range
                .filter(|(target_transform, _)| {
                    Vec3::distance(target_transform.translation(), projectile_spawn) <= tower.range
                })
                // order targets by distance, closest first
                .min_by_key(|(target_transform, _)| {
                    FloatOrd(Vec3::distance(
                        target_transform.translation(),
                        projectile_spawn,
                    ))
                })
                // basic target movement prediction
                .map(|(closest_target, nav_agent)| {
                    let distance = Vec3::distance(closest_target.translation(), projectile_spawn);
                    let time_to_target = distance / projectile_speed;
                    let prediction_vector =
                        closest_target.forward() * nav_agent.move_speed * time_to_target;

                    // predicted location - projectile spawn = projectile direction vector
                    return closest_target.translation() + prediction_vector - projectile_spawn;
                });

            if let Some(bullet_direction) = projectile_direction {
                commands.entity(tower_entity).with_children(|commands| {
                    commands.spawn((
                        SceneBundle {
                            scene: game_assets.cannon_ball_scene.clone(),
                            transform: Transform::from_translation(tower.projectile_offset),
                            ..default()
                        },
                        Name::new("Bullet"),
                        Lifetime {
                            timer: Timer::from_seconds(10.0, TimerMode::Once),
                        },
                        Projectile {
                            direction: bullet_direction,
                            speed: projectile_speed,
                            damage: 1,
                        },
                        PhysicsBundle::moving_entity_cube(Vec3::new(0.2, 0.2, 0.)),
                    ));
                });
            }
        }
    }
}
