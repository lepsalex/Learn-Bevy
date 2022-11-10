use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_game_assets)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_ui_assets);
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

pub struct UiAssets {
    pub tower_cannon_icon: Handle<Image>,
    pub tower_catapult_icon: Handle<Image>,
    pub tower_blaster_icon: Handle<Image>,
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

fn load_ui_assets(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(UiAssets {
        tower_cannon_icon: assets.load("image/tower-cannon.png"),
        tower_catapult_icon: assets.load("image/tower-catapult.png"),
        tower_blaster_icon: assets.load("image/tower-blaster.png"),
    });
}
