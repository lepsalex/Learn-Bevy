use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_mod_picking::{Hover, PickableBundle};

use crate::{assets::GameAssets, common::Despawn, TowerType};

pub struct BuilderPlugin;

impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BuildLocation>()
            .register_type::<TowerBuilder>()
            .add_system(tower_button_clicked)
            .add_system(mark_build_locations)
            .add_system(show_builder_on_hover_enter)
            .add_system(hide_builder_on_hover_leave);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct TowerBuilder {
    tower_type: TowerType,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct TowerBuilderHover;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct TowerBuilderPlacementModel;

fn tower_button_clicked(
    mut commands: Commands,
    interaction: Query<(&Interaction, &TowerType), Changed<Interaction>>,
    builder: Query<Entity, With<TowerBuilder>>,
) {
    for (interaction, tower_type) in &interaction {
        if matches!(interaction, Interaction::Clicked) {
            // Mark old builder for removal
            for entity in builder.iter() {
                commands.entity(entity).insert(Despawn);
            }

            // Create new builder
            commands
                .spawn()
                .insert(TowerBuilder {
                    tower_type: tower_type.clone(),
                })
                .insert(Name::new("Tower Builder"));
        }
    }
}

fn show_builder_on_hover_enter(
    mut commands: Commands,
    build_tile_hovered: Query<
        (Entity, &Hover),
        (
            With<BuildLocation>,
            Without<TowerBuilderHover>,
            Changed<Interaction>,
        ),
    >,
    builder: Query<&TowerBuilder>,
    game_assets: Res<GameAssets>,
) {
    // If we don't have an active builder, exit
    if builder.is_empty() {
        return;
    }

    // Place the selected tower at the build location
    for (entity, hover) in build_tile_hovered.iter() {
        if hover.hovered() {
            commands
                .entity(entity)
                .insert(TowerBuilderHover)
                .with_children(|cmd| {
                    cmd.spawn_bundle(SceneBundle {
                        scene: match builder.single().tower_type {
                            TowerType::Cannon => game_assets.tower_cannon_scene.clone(),
                            TowerType::Catapult => game_assets.tower_catapult_scene.clone(),
                            TowerType::Blaster => game_assets.tower_blaster_scene.clone(),
                        },
                        ..default()
                    })
                    .insert(NotShadowCaster)
                    .insert(TowerBuilderPlacementModel);
                });
        }
    }
}

fn hide_builder_on_hover_leave(
    mut commands: Commands,
    build_tile_hovered: Query<(Entity, &Hover), (With<BuildLocation>, Changed<Interaction>)>,
    build_placement_model: Query<Entity, With<TowerBuilderPlacementModel>>,
) {
    for (entity, hover) in build_tile_hovered.iter() {
        if !hover.hovered() {
            commands.entity(entity).remove::<TowerBuilderHover>();

            for model in build_placement_model.iter() {
                commands.entity(entity).remove_children(&[model]);
                commands.entity(model).insert(Despawn);
            }
        }
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct BuildLocation;

#[derive(Bundle)]
pub struct BuildLocationBundle {
    not_shadow_caster: NotShadowCaster,
    #[bundle]
    pickable_bundle: PickableBundle,
}

impl BuildLocationBundle {
    pub fn new() -> Self {
        Self {
            not_shadow_caster: NotShadowCaster,
            pickable_bundle: PickableBundle::default(),
        }
    }
}

#[derive(Component, Debug)]
pub struct MarkedBuildLocation;

fn mark_build_locations(
    mut commands: Commands,
    bases_query: Query<Entity, (With<BuildLocation>, Without<MarkedBuildLocation>)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for base in bases_query.iter() {
        commands
            .entity(base)
            .insert(meshes.add(Mesh::from(shape::Plane { size: 1.0 })))
            .insert_bundle(BuildLocationBundle::new())
            .insert(MarkedBuildLocation);
    }
}
