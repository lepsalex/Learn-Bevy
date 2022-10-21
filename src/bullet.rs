use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Bullet {
    pub direction: Vec3,
    pub speed: f32,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Bullet>()
            .add_system(move_bullets)
            .add_system(bullet_collision_detection);
    }
}

fn move_bullets(mut bullets: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
    }
}

fn bullet_collision_detection(
    mut commands: Commands,
    bullet_query: Query<Entity, With<Bullet>>,
    mut colliding_entities_query: Query<(&mut Health, &CollidingEntities), With<Target>>,
) {
    for (mut health, colliding_entities) in colliding_entities_query.iter_mut() {
        for bullet_entity in bullet_query.iter() {
            if colliding_entities.contains(bullet_entity) {
                commands.entity(bullet_entity).despawn_recursive();
                // TODO: get bullet damage from bullet?
                health.value -= 1;
            }
        }
    }
}
