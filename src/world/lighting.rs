use bevy::prelude::*;
pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_lighting);
    }
}

fn spawn_lighting(mut commands: Commands) {
    // Spawn Main Light
    const HALF_SIZE: f32 = 20.0;
    commands
        .spawn_bundle(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 20000.0,
                // Configure the projection to better fit the scene
                shadow_projection: OrthographicProjection {
                    left: -HALF_SIZE,
                    right: HALF_SIZE,
                    bottom: -HALF_SIZE,
                    top: HALF_SIZE,
                    near: -10.0 * HALF_SIZE,
                    far: 10.0 * HALF_SIZE,
                    ..default()
                },
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 5.0, 0.0),
                rotation: Quat::from_euler(EulerRot::XYZ, -2.25, 0.5, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Main Light"));
}
