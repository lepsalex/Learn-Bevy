use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_game_assets);
    }
}

pub struct GameAssets {
    pub level_0: Handle<Scene>,
    pub tower_scene: Handle<Scene>,
    pub tower_cannon_scene: Handle<Scene>,
    pub tower_catapult_scene: Handle<Scene>,
    pub tower_blaster_scene: Handle<Scene>,
    pub cannon_ball_scene: Handle<Scene>,
    pub ufo_red_scene: Handle<Scene>,
}

fn load_game_assets(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        level_0: assets.load("model/Level_0.glb#Scene0"),
        tower_scene: assets.load("model/Tower.glb#Scene0"),
        tower_cannon_scene: assets.load("model/TowerCannon.glb#Scene0"),
        tower_catapult_scene: assets.load("model/TowerCatapult.glb#Scene0"),
        tower_blaster_scene: assets.load("model/TowerBlaster.glb#Scene0"),
        cannon_ball_scene: assets.load("model/CannonBall.glb#Scene0"),
        ufo_red_scene: assets.load("model/UfoRed.glb#Scene0"),
    });
}
