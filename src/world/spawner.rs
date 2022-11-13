use bevy::prelude::*;

use crate::{
    assets::GameAssets,
    navigation::{NavRoute, WAYPOINT_OFFSET},
    *,
};

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
    pub enemy_type: EnemyType,
    pub spawn_timer: Timer,
    pub max_spawns: u32,
    pub num_spawned: u32,
}

#[derive(Component)]
pub struct SpawnPointDisabled;

pub fn get_spawn_point_for_enemy_type(id: u32, enemy_type: EnemyType) -> SpawnPoint {
    SpawnPoint {
        id,
        enemy_type,
        spawn_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        max_spawns: 99,
        ..default()
    }
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
            spawn_enemy(
                &mut commands,
                sp_transform.translation() + WAYPOINT_OFFSET,
                spawn_point.enemy_type,
                &nav_route.route,
                &game_assets,
            );

            // increment spawn count
            spawn_point.num_spawned += 1;
        }
    }
}
