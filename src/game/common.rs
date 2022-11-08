use bevy::prelude::*;
use bevy_mod_picking::Selection;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Lifetime>()
            .register_type::<Health>()
            .register_type::<Target>()
            .add_system(selection_debug_logging)
            .add_system(lifetime)
            .add_system(death)
            .add_system_to_stage(CoreStage::PostUpdate, despawn);
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: i32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Despawn;

fn selection_debug_logging(selection: Query<(&Name, &Selection)>) {
    for (name, selection) in &selection {
        if selection.selected() {
            info!("{} is selected", name);
        }
    }
}

fn death(mut commands: Commands, targets: Query<(Entity, &Health)>) {
    for (ent, health) in &targets {
        if health.value <= 0 {
            commands.entity(ent).insert(Despawn);
        }
    }
}

fn lifetime(
    mut commands: Commands,
    mut lifetimes: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut lifetimes {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).insert(Despawn);
        }
    }
}

fn despawn(mut commands: Commands, entities: Query<Entity, With<Despawn>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
