use bevy::{pbr::NotShadowCaster, prelude::*, ui::FocusPolicy};
use bevy_mod_picking::{Hover, PickableMesh};
use leafwing_input_manager::prelude::*;

use crate::{assets::GameAssets, common::Despawn, input::Action, spawn_tower, TowerType};

pub struct BuilderPlugin;

impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BuildLocation>()
            .register_type::<PickableBuildLocation>()
            .register_type::<Builder>()
            .register_type::<BuilderHover>()
            .register_type::<BuilderBox>()
            .add_startup_system_to_stage(StartupStage::PreStartup, load_builder_assets)
            .add_system(builder)
            .add_system(mark_build_locations)
            .add_system(show_builder_box_on_hover_enter)
            .add_system(hide_builder_box_on_hover_leave)
            .add_system(cancel_build)
            .add_system(confirm_build);
    }
}

/*
   BUILDER SPECIFIC ASSETS (ON STARTUP)
*/
pub struct BuilderAssets {
    pub build_location: Handle<Mesh>,
    pub builder_box: Handle<Mesh>,
    pub builder_box_color: Handle<StandardMaterial>,
}

fn load_builder_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(BuilderAssets {
        build_location: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
        builder_box: meshes.add(Mesh::from(shape::Cube { size: 0.95 })),
        builder_box_color: materials.add(Color::rgba(0.49, 0.97, 1.0, 0.6).into()),
    });
}

/*
    BUILD LOCATION => PICKABLE BUILD LOCATION
*/

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct BuildLocation;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct PickableBuildLocation;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct LocationBuilt;

#[derive(Bundle)]
pub struct PickableBuildLocationBundle {
    pickable_build_location: PickableBuildLocation,
    build_location: Handle<Mesh>,
    not_shadow_caster: NotShadowCaster,
    pickable_mesh: PickableMesh,
    interaction: Interaction,
    focus_policy: FocusPolicy,
    hover: Hover,
}

impl PickableBuildLocationBundle {
    pub fn new(assets: &Res<BuilderAssets>) -> Self {
        Self {
            pickable_build_location: PickableBuildLocation,
            build_location: assets.build_location.clone(),
            not_shadow_caster: NotShadowCaster,
            pickable_mesh: PickableMesh::default(),
            interaction: Interaction::default(),
            focus_policy: FocusPolicy::default(),
            hover: Hover::default(),
        }
    }
}

fn mark_build_locations(
    mut commands: Commands,
    build_locations_query: Query<Entity, (With<BuildLocation>, Without<PickableBuildLocation>)>,
    assets: Res<BuilderAssets>,
) {
    for base in build_locations_query.iter() {
        commands
            .entity(base)
            .insert_bundle(PickableBuildLocationBundle::new(&assets));
    }
}

/*
    BUILDER
*/

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Builder {
    tower_type: TowerType,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct BuilderHover;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct BuilderBox;

#[derive(Bundle, Reflect, Default)]
pub struct BuilderBoxBundle {
    builder_box: BuilderBox,
    #[bundle]
    #[reflect(ignore)]
    pbr_bundle: PbrBundle,
    no_shadow_caster: NotShadowCaster,
}

impl BuilderBoxBundle {
    pub fn new(assets: &Res<BuilderAssets>) -> Self {
        Self {
            builder_box: BuilderBox,
            pbr_bundle: PbrBundle {
                mesh: assets.builder_box.clone(),
                material: assets.builder_box_color.clone(),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            },
            no_shadow_caster: NotShadowCaster,
        }
    }
}

fn builder(
    mut commands: Commands,
    interaction: Query<(&Interaction, &TowerType), Changed<Interaction>>,
    builder: Query<Entity, With<Builder>>,
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
                .insert(Builder {
                    tower_type: tower_type.clone(),
                })
                .insert_bundle(InputManagerBundle::<Action> {
                    // Stores "which actions are currently pressed"
                    action_state: ActionState::default(),
                    // Describes how to convert from player inputs into those actions
                    input_map: InputMap::new([
                        (MouseButton::Left, Action::BuildTowerConfirm),
                        (MouseButton::Right, Action::BuildTowerCancel),
                    ]),
                })
                .insert(Name::new("Builder"));
        }
    }
}

fn show_builder_box_on_hover_enter(
    mut commands: Commands,
    build_tile_hovered: Query<
        (Entity, &Hover),
        (
            With<PickableBuildLocation>,
            Without<BuilderHover>,
            Without<LocationBuilt>,
            Changed<Interaction>,
        ),
    >,
    builder: Query<&Builder>,
    assets: Res<BuilderAssets>,
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
                .insert(BuilderHover)
                .with_children(|cmd| {
                    cmd.spawn_bundle(BuilderBoxBundle::new(&assets));
                });
        }
    }
}

fn hide_builder_box_on_hover_leave(
    mut commands: Commands,
    build_tile_hovered: Query<
        (Entity, &Hover),
        (
            With<PickableBuildLocation>,
            Without<LocationBuilt>,
            Changed<Interaction>,
        ),
    >,
    build_placement_model: Query<Entity, With<BuilderBox>>,
) {
    for (entity, hover) in build_tile_hovered.iter() {
        if !hover.hovered() {
            remove_hover(&mut commands, entity, &build_placement_model);
        }
    }
}

fn cancel_build(
    mut commands: Commands,
    builder_query: Query<Entity, With<Builder>>,
    builder_action_query: Query<&ActionState<Action>, With<Builder>>,
    build_placement_model: Query<Entity, With<BuilderBox>>,
) {
    // If we don't have a builder or builder_actions, exit
    if builder_query.is_empty() || builder_action_query.is_empty() {
        return;
    }

    let entity = builder_query.single();
    let action_state = builder_action_query.single();

    if action_state.just_pressed(Action::BuildTowerCancel) {
        // remove hover components and entities
        remove_hover(&mut commands, entity, &build_placement_model);

        // despawn builder
        commands.entity(entity).insert(Despawn);
    }
}

fn confirm_build(
    mut commands: Commands,
    builder_query: Query<(Entity, &Builder)>,
    builder_action_query: Query<&ActionState<Action>, With<Builder>>,
    build_tile_hovered: Query<
        (Entity, &Hover, &Transform),
        (With<PickableBuildLocation>, Without<LocationBuilt>),
    >,
    build_placement_model: Query<Entity, With<BuilderBox>>,
    assets: Res<GameAssets>,
) {
    // If we don't have a builder or builder_actions, exit
    if builder_query.is_empty() || builder_action_query.is_empty() {
        return;
    }

    let (builder_entity, builder) = builder_query.single();
    let action_state = builder_action_query.single();

    if action_state.just_pressed(Action::BuildTowerConfirm) {
        for (entity, hover, transform) in build_tile_hovered.iter() {
            if hover.hovered() {
                // remove hover components and entities
                remove_hover(&mut commands, entity, &build_placement_model);

                // insert LocationBuilt and remove marked build location
                commands
                    .entity(entity)
                    .insert(LocationBuilt)
                    .remove_bundle::<PickableBuildLocationBundle>();

                // spawn the tower
                spawn_tower(
                    &mut commands,
                    builder.tower_type,
                    &assets,
                    transform.translation,
                );

                // remove the builder
                commands.entity(builder_entity).insert(Despawn);
            }
        }
    }
}

fn remove_hover(
    commands: &mut Commands,
    entity: Entity,
    build_placement_model: &Query<Entity, With<BuilderBox>>,
) {
    commands.entity(entity).remove::<BuilderHover>();
    for model in build_placement_model.iter() {
        commands.entity(entity).remove_children(&[model]);
        commands.entity(model).insert(Despawn);
    }
}
