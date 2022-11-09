use bevy::prelude::*;

use crate::{assets::UiAssets, TowerType};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ui);
    }
}

pub struct RootUi;

fn ui(mut commands: Commands, ui_assets: Res<UiAssets>) {
    let towers = [TowerType::Cannon, TowerType::Catapult, TowerType::Blaster];

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
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
                            size: Size::new(Val::Percent(15.0 * 9.0 / 16.0), Val::Percent(15.0)),
                            align_self: AlignSelf::FlexStart,
                            margin: UiRect::all(Val::Percent(2.0)),
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
