use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode};
use bevy_mod_picking::PickingCameraBundle;
use leafwing_input_manager::prelude::*;

use crate::input::Action;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameCamera>()
            .add_startup_system(spawn_main_camera)
            .add_system(camera_controls);
    }
}

const MAIN_CAMERA_SPEED: f32 = 10.0;
const MAIN_CAMERA_ROTATION_SPEED: f32 = 3.0;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct GameCamera {}

fn spawn_main_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::hsl(209.0, 0.45, 0.22)),
                ..default()
            },
            transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            projection: OrthographicProjection {
                scale: 10.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
            .into(),
            ..default()
        },
        PickingCameraBundle::default(),
        InputManagerBundle::<Action> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map: InputMap::new([
                (KeyCode::W, Action::CameraMoveForward),
                (KeyCode::S, Action::CameraMoveBackward),
                (KeyCode::A, Action::CameraMoveLeft),
                (KeyCode::D, Action::CameraMoveRight),
                // (KeyCode::Q, Action::CameraRotateLeft),
                // (KeyCode::E, Action::CameraRotateRight),
            ]),
        },
        GameCamera {},
        Name::new("Main Camera"),
    ));
}

fn camera_controls(
    action: Query<&ActionState<Action>, With<GameCamera>>,
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
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

    let action_state = action.single();

    // Handle Camera Movement Input
    if action_state.pressed(Action::CameraMoveForward) {
        camera.translation += forward * time.delta_seconds() * MAIN_CAMERA_SPEED;
    }
    if action_state.pressed(Action::CameraMoveBackward) {
        camera.translation -= forward * time.delta_seconds() * MAIN_CAMERA_SPEED;
    }
    if action_state.pressed(Action::CameraMoveLeft) {
        camera.translation += left * time.delta_seconds() * MAIN_CAMERA_SPEED;
    }
    if action_state.pressed(Action::CameraMoveRight) {
        camera.translation -= left * time.delta_seconds() * MAIN_CAMERA_SPEED;
    }

    // Handle Camera Rotation Input
    if action_state.pressed(Action::CameraRotateLeft) {
        camera.rotate_axis(Vec3::Y, MAIN_CAMERA_ROTATION_SPEED * time.delta_seconds())
    }
    if action_state.pressed(Action::CameraRotateRight) {
        camera.rotate_axis(Vec3::Y, -MAIN_CAMERA_ROTATION_SPEED * time.delta_seconds())
    }
}
