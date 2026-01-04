use crate::components::{FollowCamera, Player};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub fn follow_player(
    player_query: Query<&Transform, (With<Player>, Without<FollowCamera>)>,
    mut camera_query: Query<(&mut Transform, &FollowCamera), With<FollowCamera>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.single() {
        for (mut camera_transform, follow_camera) in &mut camera_query {
            let offset = Vec3::new(
                follow_camera.distance * follow_camera.yaw.cos() * follow_camera.pitch.cos(),
                follow_camera.distance * follow_camera.pitch.sin() + follow_camera.height,
                follow_camera.distance * follow_camera.yaw.sin() * follow_camera.pitch.cos(),
            );

            let target_position = player_transform.translation + offset;

            camera_transform.translation = camera_transform.translation.lerp(
                target_position,
                time.delta_secs() * follow_camera.smoothness,
            );

            camera_transform.look_at(player_transform.translation, Vec3::Y);
        }
    }
}

pub fn camera_mouse_control(
    mut mouse_motion: MessageReader<MouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<&mut FollowCamera>,
) {
    if !mouse_button.pressed(MouseButton::Left) {
        return;
    }

    for mut camera in &mut camera_query {
        for motion in mouse_motion.read() {
            camera.yaw += motion.delta.x * camera.sensitivity;
            camera.pitch += motion.delta.y * camera.sensitivity;

            camera.pitch = camera.pitch.clamp(-1.5, 1.5);
        }
    }
}
