use bevy::{prelude::*, utils::HashMap};

use crate::spawner::SpawnPoint;

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<NavAgent>()
            .register_type::<Waypoint>()
            .add_system_to_stage(CoreStage::PreUpdate, init_nav_routes)
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

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct NavRoute {
    pub route: Vec<Vec3>,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Waypoint {
    pub id: u32,
    pub spawn_id: u32,
}

#[derive(Component)]
pub struct NavigationInitiated;

pub const WAYPOINT_OFFSET: Vec3 = Vec3::new(0.0, 0.5, 0.0);

fn init_nav_routes(
    mut commands: Commands,
    waypoints_query: Query<(Entity, &Waypoint, &GlobalTransform), Without<NavigationInitiated>>,
    spawn_points_query: Query<(Entity, &SpawnPoint), Without<NavigationInitiated>>,
) {
    // Group waypoints by spawn id
    let grouped: HashMap<u32, Vec<(u32, Vec3)>> = waypoints_query
        .into_iter()
        .map(|(entity, wp, t)| {
            // mark this wp as initiated
            commands.entity(entity).insert(NavigationInitiated);

            // return a tuple of (spawn_id, (wp_id, wp_location))
            return (wp.spawn_id, (wp.id, t.translation() + WAYPOINT_OFFSET));
        })
        .fold(
            HashMap::default(),
            |mut map, (spawn_id, (wp_id, location))| {
                // Fold into a HashMap of <spawn_id, Vec<(wp_id, wp_loc)>

                match map.get_key_value_mut(&spawn_id) {
                    Some((_, route)) => route.push((wp_id, location)),
                    None => {
                        map.insert(spawn_id, Vec::from([(wp_id, location)]));
                    }
                }

                return map;
            },
        );

    // Build routes, keyed on spawn id, sorted by waypoint id, value as Vec<Vec3>
    let mut routes: HashMap<u32, Vec<Vec3>> = HashMap::default();

    for (spawn_id, mut waypoints) in grouped {
        // Sort by waypoint id
        waypoints.sort_by(|(wp_a, _), (wp_b, _)| wp_b.cmp(&wp_a));

        routes.insert(
            spawn_id.clone(),
            waypoints
                .iter()
                .map(|(_, location)| location.clone())
                .collect(),
        );
    }

    // Attach NavRoute to SpawnPoints
    for (entity, spawn_point) in spawn_points_query.iter() {
        match routes.get(&spawn_point.id) {
            Some(route) => {
                commands
                    .entity(entity)
                    .insert(NavRoute {
                        route: route.clone(),
                    })
                    .insert(NavigationInitiated);
            }
            None => warn!("No route exists!"),
        }
    }
}

const DESTINATION_TOLERANCE: f32 = 0.2;

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
        let incremental_turn_weight = agent.turn_speed * time.delta_seconds();
        let old_rotation = transform.rotation;
        transform.rotation =
            old_rotation.lerp(look_at_destination.rotation, incremental_turn_weight);

        // Move agent in their forward direction at move speed
        let forward_move = transform.forward() * agent.move_speed * time.delta_seconds();
        transform.translation += forward_move;
    }
}
