use bevy::prelude::*;

use crate::{navigation::NavAgent, *};

#[derive(Default, Bundle, Reflect)]
pub struct EnemyBundle {
    target: Target,
    health: Health,
    nav_agent: NavAgent,
    #[bundle]
    #[reflect(ignore)]
    physics_bundle: PhysicsBundle,
}

impl EnemyBundle {
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

#[derive(Reflect, Clone, Copy, Default)]
pub enum EnemyType {
    #[default]
    EnemyBasic,
}

pub fn get_enemy_bundle(enemy_type: EnemyType, nav_route: &Vec<Vec3>) -> EnemyBundle {
    match enemy_type {
        EnemyType::EnemyBasic => EnemyBundle::new(
            3,
            0.6,
            2.4,
            nav_route.clone(),
            PhysicsBundle::moving_entity_sphere(0.6),
        ),
    }
}
