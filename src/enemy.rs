use bevy::prelude::*;

use crate::*;

#[derive(Default, Bundle, Reflect)]
pub struct EnemyBundleTemplate {
    target: Target,
    health: Health,
    nav_agent: NavAgent,
    #[bundle]
    #[reflect(ignore)]
    physics_bundle: PhysicsBundle,
}

impl EnemyBundleTemplate {
    pub fn new(
        health: i32,
        move_speed: f32,
        turn_speed: f32,
        route: Vec<Vec3>,
        physics_bundle: PhysicsBundle,
    ) -> Self {
        Self {
            target: Target,
            health: Health { value: health },
            nav_agent: NavAgent {
                move_speed,
                turn_speed,
                delay_timer: Timer::from_seconds(0.5, false),
                route,
                ..default()
            },
            physics_bundle,
        }
    }
}
