use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Bundle)]
pub struct PhysicsBundle {
    flags: ActiveEvents,
    active_collision_types: ActiveCollisionTypes,
    collider: Collider,
    colliding_entities: CollidingEntities,
    rigid_body: RigidBody,
    rotation_constraint: LockedAxes,
    velocity: Velocity,
}

impl PhysicsBundle {
    pub fn moving_entity(size: Vec3) -> Self {
        Self {
            flags: ActiveEvents::COLLISION_EVENTS,
            active_collision_types: ActiveCollisionTypes::default()
                | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
            collider: Collider::cuboid(size.x / 2., size.y / 2., size.z / 2.),
            colliding_entities: CollidingEntities::default(),
            rigid_body: RigidBody::KinematicPositionBased,
            rotation_constraint: LockedAxes::ROTATION_LOCKED,
            velocity: Velocity::zero(),
        }
    }
}
