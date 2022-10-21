use crate::*;

use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>().add_system(tower_shooting);
    }
}

fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    bullet_assets: Res<GameAssets>,
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
                        .spawn_bundle(SceneBundle {
                            scene: bullet_assets.bullet_scene.clone(),
                            transform: Transform::from_translation(tower.bullet_offset),
                            ..default()
                        })
                        .insert(Lifetime {
                            timer: Timer::from_seconds(1000.0, false),
                        })
                        .insert(Bullet {
                            direction: bullet_direction,
                            speed: 2.5,
                        })
                        .insert_bundle(PhysicsBundle::moving_entity(Vec3::new(0.2, 0.2, 0.2)))
                        .insert(Name::new("Bullet"));
                });
            }
        }
    }
}
