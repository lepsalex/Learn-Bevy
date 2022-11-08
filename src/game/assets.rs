use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, asset_loading);
    }
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
        level_0: assets.load("model/Level_0.glb#Scene0"),
        tower_scene: assets.load("model/Tower.glb#Scene0"),
        cannon_ball_scene: assets.load("model/CannonBall.glb#Scene0"),
        ufo_red_scene: assets.load("model/UfoRed.glb#Scene0"),
        tower_base_mesh: assets.load("model/TowerBase.glb#Mesh0/Primitive0"),
    });
}
