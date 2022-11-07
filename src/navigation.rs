use bevy::prelude::*;

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<NavAgent>()
            .add_system_to_stage(CoreStage::PreUpdate, nav_agent_delay)
            .add_system(move_nav_agents);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct NavAgent {
    pub speed: f32,
    pub delay_timer: Timer,
}

#[derive(Component)]
pub struct NavAgentEnabled;

fn nav_agent_delay(
    mut commands: Commands,
    mut agents: Query<(Entity, &mut NavAgent), Without<NavAgentEnabled>>,
    time: Res<Time>,
) {
    for (entity, mut agent) in &mut agents {
        agent.delay_timer.tick(time.delta());

        if agent.delay_timer.just_finished() {
            commands.entity(entity).insert(NavAgentEnabled);
        }
    }
}

fn move_nav_agents(
    mut agents: Query<(&NavAgent, &mut Transform), With<NavAgentEnabled>>,
    time: Res<Time>,
) {
    for (agent, mut transform) in &mut agents {
        transform.translation.x += agent.speed * time.delta_seconds();
    }
}
