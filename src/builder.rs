use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_mod_picking::{Hover, PickableBundle};

use crate::{common::Despawn, TowerType};

pub struct BuilderPlugin;

impl Plugin for BuilderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BuildLocation>()
            .register_type::<AvailableBuildLocation>()
            .register_type::<Builder>()
            .register_type::<BuilderHover>()
            .register_type::<BuilderBox>()
            .add_startup_system_to_stage(StartupStage::PreStartup, load_builder_assets)
            .add_system(builder)
            .add_system(mark_build_locations)
            .add_system(show_builder_box_on_hover_enter)
            .add_system(hide_builder_box_on_hover_leave);
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
    BUILD LOCATION => AVAILABLE BUILD LOCATION
*/

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct BuildLocation;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct AvailableBuildLocation;

#[derive(Bundle)]
pub struct MarkedBuildLocationBundle {
    marked_build_location: AvailableBuildLocation,
    build_location: Handle<Mesh>,
    not_shadow_caster: NotShadowCaster,
    #[bundle]
    pickable_bundle: PickableBundle,
}

impl MarkedBuildLocationBundle {
    pub fn new(assets: &Res<BuilderAssets>) -> Self {
        Self {
            marked_build_location: AvailableBuildLocation,
            build_location: assets.build_location.clone(),
            not_shadow_caster: NotShadowCaster,
            pickable_bundle: PickableBundle::default(),
        }
    }
}

fn mark_build_locations(
    mut commands: Commands,
    build_locations_query: Query<Entity, (With<BuildLocation>, Without<AvailableBuildLocation>)>,
    assets: Res<BuilderAssets>,
) {
    for base in build_locations_query.iter() {
        commands
            .entity(base)
            .insert_bundle(MarkedBuildLocationBundle::new(&assets));
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
                .insert(Name::new("Tower Builder"));
        }
    }
}

fn show_builder_box_on_hover_enter(
    mut commands: Commands,
    build_tile_hovered: Query<
        (Entity, &Hover),
        (
            With<AvailableBuildLocation>,
            Without<BuilderHover>,
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
        (With<AvailableBuildLocation>, Changed<Interaction>),
    >,
    build_placement_model: Query<Entity, With<BuilderBox>>,
) {
    for (entity, hover) in build_tile_hovered.iter() {
        if !hover.hovered() {
            commands.entity(entity).remove::<BuilderHover>();

            for model in build_placement_model.iter() {
                commands.entity(entity).remove_children(&[model]);
                commands.entity(model).insert(Despawn);
            }
        }
    }
}
