pub use game::*;

use bevy::prelude::*;

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
            .add_system(bullet_collision);
    }
}

fn move_bullets(mut bullets: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
    }
}

fn bullet_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &GlobalTransform), With<Bullet>>,
    mut targets: Query<(&mut Health, &Transform), With<Target>>,
) {
    // simplistic collision system, consider physics engine
    for (bullet, bullet_transform) in &bullets {
        for (mut health, target_transform) in &mut targets {
            // TODO: magic number for collision distance
            if Vec3::distance(bullet_transform.translation(), target_transform.translation) < 0.2 {
                commands.entity(bullet).despawn_recursive();
                health.value -= 1;
                break;
            }
        }
    }
}
