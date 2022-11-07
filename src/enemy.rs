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
        speed: f32,
        physics_bundle: PhysicsBundle,
    ) -> Self {
        Self {
            target: Target,
            health: Health { value: health },
            nav_agent: NavAgent {
                speed: speed,
                delay_timer: Timer::from_seconds(0.5, false),
            },
            physics_bundle,
        }
    }
}
