use bevy::prelude::*;

use crate::*;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SpawnPoint>()
            .add_system_to_stage(CoreStage::Update, spawner);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct SpawnPoint {
    pub id: u32,
    pub spawn_entity: SpawnEntityMapping,
    pub spawn_timer: Timer,
    pub max_spawns: u32,
    pub num_spawned: u32,
}

#[derive(Component)]
pub struct SpawnPointDisabled;

#[derive(Reflect, Clone, Copy, Default)]
pub enum SpawnEntityMapping {
    #[default]
    EnemyBasic,
}

fn spawner(
    mut commands: Commands,
    mut spawn_points: Query<
        (Entity, &mut SpawnPoint, &NavRoute, &GlobalTransform),
        Without<SpawnPointDisabled>,
    >,
    game_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for (entity, mut spawn_point, nav_route, sp_transform) in &mut spawn_points {
        // if spawn point is maxed, mark and exit early
        if spawn_point.num_spawned >= spawn_point.max_spawns {
            commands.entity(entity).insert(SpawnPointDisabled);
            return;
        }

        // tick the spawn point timer
        spawn_point.spawn_timer.tick(time.delta());

        // spawn an entity at the spawn point if the timer just finished
        if spawn_point.spawn_timer.just_finished() {
            match spawn_point.spawn_entity {
                SpawnEntityMapping::EnemyBasic => {
                    commands
                        .spawn_bundle(SpatialBundle {
                            transform: Transform::from_translation(
                                sp_transform.translation() + WAYPOINT_OFFSET,
                            )
                            .looking_at(*nav_route.route.last().unwrap(), Vec3::Y),
                            ..default()
                        })
                        .insert(Name::new("Enemy (basic)"))
                        .insert_bundle(EnemyBundleTemplate::new(
                            3,
                            0.6,
                            2.4,
                            nav_route.route.clone(),
                            PhysicsBundle::moving_entity_sphere(0.6),
                        ))
                        .with_children(|commands| {
                            commands.spawn_bundle(SceneBundle {
                                scene: game_assets.ufo_red_scene.clone(),
                                ..default()
                            });
                        });
                }
            }

            // increment spawn count
            spawn_point.num_spawned += 1;
        }
    }
}
