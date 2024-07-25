use bevy::{input::mouse::*, prelude::*};

use super::{resources::*, utils::*};

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn((
        MainCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Dir3::Y),
            ..default()
        },
    ));
}

pub fn first_person_camera_control(
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    mut evr_mouse_motion: EventReader<MouseMotion>,
    movement_settings: Res<MovementSettings>,
) {
    let sensitivity = movement_settings.camera_sensitivity;

    let mut transform = q_camera.get_single_mut().unwrap();
    for ev in evr_mouse_motion.read() {
        let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

        yaw -= (ev.delta.x * sensitivity).to_radians();
        pitch -= (ev.delta.y * sensitivity).to_radians();

        pitch = pitch.clamp(-1.54, 1.54);

        transform.rotation =
            Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
    }
}

pub fn free_camera_control(
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    mut evr_mouse_motion: EventReader<MouseMotion>,
    keys: Res<ButtonInput<KeyCode>>,
    movement_settings: Res<MovementSettings>,
) {
    let sensitivity = movement_settings.camera_sensitivity;

    let mut transform = q_camera.get_single_mut().unwrap();
    for ev in evr_mouse_motion.read() {
        let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

        yaw -= (ev.delta.x * sensitivity).to_radians();
        pitch -= (ev.delta.y * sensitivity).to_radians();

        pitch = pitch.clamp(-1.54, 1.54);

        transform.rotation =
            Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
    }

    let forward = transform.forward();
    let right = transform.right();
    let mut direction = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        direction += to_vec(forward);
    }
    if keys.pressed(KeyCode::KeyS) {
        direction -= to_vec(forward);
    }
    if keys.pressed(KeyCode::KeyA) {
        direction -= to_vec(right);
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += to_vec(right);
    }
    if keys.pressed(KeyCode::Space) {
        direction += Vec3::Y;
    }
    if keys.pressed(KeyCode::ShiftLeft) {
        direction -= Vec3::Y;
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
        transform.translation += direction * movement_settings.player_movement_speed;
    }
}
