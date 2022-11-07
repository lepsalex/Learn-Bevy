use bevy::prelude::*;

use crate::*;

#[derive(Default, Bundle, Reflect)]
pub struct EnemyBundleTemplate {
    target: Target,
    health: Health,
    #[bundle]
    #[reflect(ignore)]
    physics_bundle: PhysicsBundle,
}

impl EnemyBundleTemplate {
    pub fn new(
        target: Target,
        health: Health,
        physics_bundle: PhysicsBundle,
    ) -> Self {
        Self {
            target,
            health,
            physics_bundle,
        }
    }
}
