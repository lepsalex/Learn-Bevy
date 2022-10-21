use bevy::prelude::*;

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
    pub tower_base_scene: Handle<Scene>,
    pub tomato_tower_scene: Handle<Scene>,
    pub tomato_scene: Handle<Scene>,
    pub target_scene: Handle<Scene>,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Lifetime>()
            .register_type::<Health>()
            .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
            .add_system(entity_despawn)
            .add_system(death);
    }
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        tower_base_scene: assets.load("TowerBase.glb#Scene0"),
        tomato_tower_scene: assets.load("TomatoTower.glb#Scene0"),
        tomato_scene: assets.load("Tomato.glb#Scene0"),
        target_scene: assets.load("Target.glb#Scene0"),
    });
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
