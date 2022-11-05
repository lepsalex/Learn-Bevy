use crate::*;

use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_mod_picking::{Highlighting, PickableBundle};

pub struct TowerBasePlugin;

impl Plugin for TowerBasePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TowerBaseLocation>()
            .add_system(spawn_tower_base_locations);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct TowerBaseLocation;

#[derive(Component, Debug)]
pub struct TowerBaseLocationSpawned;

#[derive(Bundle)]
pub struct TowerBaseLocationBundle {
    mesh: Handle<Mesh>,
    highlighting: Highlighting<StandardMaterial>,
    material: Handle<StandardMaterial>,
    not_shadow_caster: NotShadowCaster,
    #[bundle]
    pickable_bundle: PickableBundle,
}

impl TowerBaseLocationBundle {
    pub fn new(
        mesh: &Handle<Mesh>,
        initial_color: &Handle<StandardMaterial>,
        highlight_color: &Handle<StandardMaterial>,
    ) -> Self {
        Self {
            mesh: mesh.clone(),
            highlighting: Highlighting {
                initial: initial_color.clone(),
                hovered: Some(highlight_color.clone()),
                pressed: Some(highlight_color.clone()),
                selected: Some(highlight_color.clone()),
            },
            material: initial_color.clone(),
            not_shadow_caster: NotShadowCaster,
            pickable_bundle: PickableBundle::default(),
        }
    }
}

fn spawn_tower_base_locations(
    mut commands: Commands,
    bases_query: Query<Entity, (With<TowerBaseLocation>, Without<TowerBaseLocationSpawned>)>,
    game_assets: Res<GameAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let default_color = materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into());
    let selected_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.5).into());

    for base in bases_query.iter() {
        commands
            .entity(base)
            .insert(Name::new("Tower_Base_Location"))
            .insert_bundle(TowerBaseLocationBundle::new(
                &game_assets.tower_base_mesh,
                &default_color,
                &selected_color,
            ))
            .insert(TowerBaseLocationSpawned);
    }
}
