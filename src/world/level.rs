use bevy::prelude::*;
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

use crate::{GameAssets, TowerBaseLocation};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SpawnPoint>()
            .register_type::<Waypoint>()
            .add_startup_system(spawn_level);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct SpawnPoint {
    pub id: u32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Waypoint {
    pub id: u32,
    pub spawn_id: u32,
}

fn spawn_level(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // // TODO: Make this tile
    // let blg_material_handle = materials.add(StandardMaterial {
    //     base_color_texture: Some(game_assets.bkg_tile.clone()),
    //     alpha_mode: AlphaMode::Blend,
    //     unlit: true,
    //     ..default()
    // });

    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
    //     material: blg_material_handle,
    //     ..default()
    // });

    // Spawn Level
    commands
        .spawn_bundle(HookedSceneBundle {
            scene: SceneBundle {
                scene: game_assets.level_0.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            hook: SceneHook::new(|entity, cmds| {
                entity.get::<Name>().map(|name| {
                    /*
                    Attach required components for marked tiles
                    */
                    if name.starts_with("grass") {
                        cmds.insert(TowerBaseLocation);
                    }

                    if name.starts_with("spawn") {
                        let data: Vec<&str> = name.split(".").collect();
                        cmds.insert(SpawnPoint {
                            id: data.get(1).unwrap().parse::<u32>().unwrap(),
                        });
                    }

                    if name.starts_with("waypoint") {
                        let data: Vec<&str> = name.split(".").collect();
                        cmds.insert(Waypoint {
                            id: data.get(1).unwrap().parse::<u32>().unwrap(),
                            spawn_id: data.get(2).unwrap().parse::<u32>().unwrap(),
                        });
                    }
                });
            }),
        })
        .insert(Name::new("Level"));
}
