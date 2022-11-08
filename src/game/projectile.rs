use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::common::{Despawn, Health, Target};

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Projectile {
    pub direction: Vec3,
    pub speed: f32,
    pub damage: i32,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Projectile>()
            .add_system(move_projectiles)
            .add_system(projectile_collision_detection);
    }
}

fn move_projectiles(mut projectiles: Query<(&Projectile, &mut Transform)>, time: Res<Time>) {
    for (projectile, mut transform) in &mut projectiles {
        transform.translation +=
            projectile.direction.normalize() * projectile.speed * time.delta_seconds();
    }
}

fn projectile_collision_detection(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Projectile)>,
    mut colliding_entities_query: Query<(&mut Health, &CollidingEntities), With<Target>>,
) {
    for (mut health, colliding_entities) in colliding_entities_query.iter_mut() {
        for (projectile_entity, projectile) in projectile_query.iter() {
            if colliding_entities.contains(projectile_entity) {
                commands.entity(projectile_entity).insert(Despawn);
                health.value -= projectile.damage;
            }
        }
    }
}
