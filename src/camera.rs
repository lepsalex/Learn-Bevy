use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_main_camera)
        .add_system(camera_controls);
    }
}

const MAIN_CAMERA_SPEED: f32 = 3.0;
const MAIN_CAMERA_ROTATION_SPEED: f32 = 1.0;

fn spawn_main_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Name::new("Main Camera"));
}

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    // WARNING: This will panic if we ever spawn more than one camera!
    let mut camera = camera_query.single_mut();
    
    // Get camera forward vector (Y zero'd to account for down angle)
    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    // Get camera left vector (Y zero'd to account for down angle)
    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    // Handle Camera Movement Input
    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * MAIN_CAMERA_SPEED;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * MAIN_CAMERA_SPEED;
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * MAIN_CAMERA_SPEED;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * MAIN_CAMERA_SPEED;
    }

    // Handle Camera Rotation Input
    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, MAIN_CAMERA_ROTATION_SPEED * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -MAIN_CAMERA_ROTATION_SPEED * time.delta_seconds())
    }
}
