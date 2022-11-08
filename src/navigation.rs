use bevy::prelude::*;

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<NavAgent>()
            .add_system_to_stage(CoreStage::PreUpdate, init_nav_agents)
            .add_system_to_stage(CoreStage::PreUpdate, update_nav_agent_destination)
            .add_system(move_nav_agents);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct NavAgent {
    pub move_speed: f32,
    pub turn_speed: f32,
    pub delay_timer: Timer,
    pub route: Vec<Vec3>,
    pub destination: Vec3,
}

#[derive(Component)]
pub struct NavAgentEnabled;

const DESTINATION_TOLERANCE: f32 = 0.05;

fn init_nav_agents(
    mut commands: Commands,
    mut agents: Query<(Entity, &mut NavAgent), Without<NavAgentEnabled>>,
    time: Res<Time>,
) {
    for (entity, mut agent) in &mut agents {
        agent.delay_timer.tick(time.delta());

        if agent.delay_timer.just_finished() && !agent.route.is_empty() {
            agent.destination = agent.route.pop().unwrap();
            commands.entity(entity).insert(NavAgentEnabled);
        }
    }
}

fn update_nav_agent_destination(
    mut commands: Commands,
    mut agents: Query<(Entity, &mut NavAgent, &Transform), With<NavAgentEnabled>>,
) {
    for (entity, mut agent, transform) in &mut agents {
        if transform.translation.distance(agent.destination) < DESTINATION_TOLERANCE {
            match agent.route.pop() {
                Some(destination) => agent.destination = destination,
                None => {
                    commands.entity(entity).remove::<NavAgentEnabled>();
                }
            }
        }
    }
}

fn move_nav_agents(
    mut agents: Query<(&NavAgent, &mut Transform), With<NavAgentEnabled>>,
    time: Res<Time>,
) {
    for (agent, mut transform) in &mut agents {
        // Rotate agent towards destination at rotation speed
        let look_at_destination = transform.looking_at(agent.destination, transform.local_y());
        let incremental_turn_weight = agent.move_speed * time.delta_seconds();
        let old_rotation = transform.rotation;
        transform.rotation =
            old_rotation.lerp(look_at_destination.rotation, incremental_turn_weight);

        // Move agent in their forward direction at move speed
        let forward_move = transform.forward() * agent.move_speed * time.delta_seconds();
        transform.translation += forward_move;
    }
}
