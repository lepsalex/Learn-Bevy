use bevy::prelude::*;
use bevy_mod_picking::Selection;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Lifetime>()
            .register_type::<Health>()
            .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
            .add_system(selection_debug_logging)
            .add_system(entity_despawn)
            .add_system(death);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: i32,
}

pub struct GameAssets {
    pub level_0: Handle<Scene>,
    pub tower_scene: Handle<Scene>,
    pub cannon_ball_scene: Handle<Scene>,
    pub ufo_red_scene: Handle<Scene>,
    pub tower_base_mesh: Handle<Mesh>,
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        level_0: assets.load("Level_0.glb#Scene0"),
        tower_scene: assets.load("Tower.glb#Scene0"),
        cannon_ball_scene: assets.load("CannonBall.glb#Scene0"),
        ufo_red_scene: assets.load("UfoRed.glb#Scene0"),
        tower_base_mesh: assets.load("TowerBase.glb#Mesh0/Primitive0"),
    });
}

fn selection_debug_logging(selection: Query<(&Name, &Selection)>) {
    for (name, selection) in &selection {
        if selection.selected() {
            info!("{} is selected", name);
        }
    }
}

fn entity_despawn(
    mut commands: Commands,
    mut lifetimes: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut lifetimes {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn death(mut commands: Commands, targets: Query<(Entity, &Health)>) {
    for (ent, health) in &targets {
        if health.value <= 0 {
            commands.entity(ent).despawn_recursive();
        }
    }
}
