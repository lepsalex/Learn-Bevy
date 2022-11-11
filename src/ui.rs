use bevy::prelude::*;

use crate::TowerType;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ui_assets)
            .add_startup_system(ui);
    }
}

/*
   UI SPECIFIC ASSETS (ON STARTUP)
*/
pub struct UiAssets {
    pub tower_cannon_icon: Handle<Image>,
    pub tower_catapult_icon: Handle<Image>,
    pub tower_blaster_icon: Handle<Image>,
}
fn load_ui_assets(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(UiAssets {
        tower_cannon_icon: assets.load("image/tower-cannon.png"),
        tower_catapult_icon: assets.load("image/tower-catapult.png"),
        tower_blaster_icon: assets.load("image/tower-blaster.png"),
    });
}

pub struct RootUi;

fn ui(mut commands: Commands, ui_assets: Res<UiAssets>) {
    let towers = [TowerType::Cannon, TowerType::Catapult, TowerType::Blaster];

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                padding: UiRect::all(Val::Px(16.0)),
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|commands| {
            for tower_type in towers {
                commands
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                            align_self: AlignSelf::FlexStart,
                            margin: UiRect::all(Val::Px(16.0)),
                            ..default()
                        },
                        image: match tower_type {
                            TowerType::Cannon => ui_assets.tower_cannon_icon.clone().into(),
                            TowerType::Catapult => ui_assets.tower_catapult_icon.clone().into(),
                            TowerType::Blaster => ui_assets.tower_blaster_icon.clone().into(),
                        },
                        ..default()
                    })
                    .insert(tower_type);
            }
        });
}
