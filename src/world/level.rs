use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_mod_picking::{Highlighting, PickableBundle};
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

use crate::GameAssets;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TowerBase>()
            .register_type::<SpawnPoint>()
            .register_type::<Waypoint>()
            .add_startup_system(spawn_level)
            .add_system(spawn_tower_bases);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct TowerBase;

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

#[derive(Component, Debug)]
pub struct LevelComponentSpawned;

fn spawn_level(mut commands: Commands, game_assets: Res<GameAssets>) {
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
                    if name.starts_with("base") {
                        cmds.insert(TowerBase);
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

fn spawn_tower_bases(
    mut commands: Commands,
    bases_query: Query<Entity, (With<TowerBase>, Without<LevelComponentSpawned>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn Tower Base
    let default_collider_color = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into());
    let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9).into());
    let collider_mesh = meshes.add(shape::Capsule::default().into());

    for base in bases_query.iter() {
        info!("Base Id: {}", base.id());

        commands
            .entity(base)
            .insert(Name::new("Tower_Base"))
            .insert(collider_mesh.clone())
            .insert(Highlighting {
                initial: default_collider_color.clone(),
                hovered: Some(selected_collider_color.clone()),
                pressed: Some(selected_collider_color.clone()),
                selected: Some(selected_collider_color.clone()),
            })
            .insert(default_collider_color.clone())
            .insert(NotShadowCaster)
            .insert(LevelComponentSpawned)
            .insert_bundle(PickableBundle::default());
    }
}
