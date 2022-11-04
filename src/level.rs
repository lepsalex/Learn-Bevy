use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_mod_picking::{Highlighting, PickableBundle};
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

use crate::GameAssets;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_level);
    }
}

fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,
) {
    // Spawn Tower Base
    let default_collider_color = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into());
    let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9).into());
    let collider_mesh = meshes.add(shape::Capsule::default().into());

    // Spawn Level
    commands
        .spawn_bundle(HookedSceneBundle {
            scene: SceneBundle {
                scene: game_assets.level_0.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            hook: SceneHook::new(move |entity, cmds| {
                entity.get::<Name>().map(|name| {
                    /*
                    Attach required components for
                    tower base within the Blender scene

                    TODO: Consider refactoring this to a module?
                    */
                    if name.starts_with("base") {
                        cmds.insert(Name::new("Tower_Base"))
                            .insert(collider_mesh.clone())
                            .insert(Highlighting {
                                initial: default_collider_color.clone(),
                                hovered: Some(selected_collider_color.clone()),
                                pressed: Some(selected_collider_color.clone()),
                                selected: Some(selected_collider_color.clone()),
                            })
                            .insert(default_collider_color.clone())
                            .insert(NotShadowCaster)
                            .insert_bundle(PickableBundle::default());
                    }
                });
            }),
        })
        .insert(Name::new("Level"));
}
