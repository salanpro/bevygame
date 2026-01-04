use crate::components::{FollowCamera, Player, Rotatable};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Velocity, With<Player>>,
    camera_query: Query<&Transform, With<FollowCamera>>,
) {
    let thrust = 0.3;
    let max_speed = 10.0;

    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    for mut velocity in &mut player_query {
        let mut acceleration = Vec3::ZERO;

        let forward = camera_transform.forward();
        let right = camera_transform.right();

        let forward_flat = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
        let right_flat = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

        if keyboard.pressed(KeyCode::KeyW) {
            acceleration += forward_flat;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            acceleration -= forward_flat;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            acceleration -= right_flat;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            acceleration += right_flat;
        }
        if keyboard.pressed(KeyCode::ArrowUp) {
            acceleration.y += 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            acceleration.y -= 1.0;
        }

        if acceleration.length() > 0.0 {
            acceleration = acceleration.normalize();
        }

        velocity.linvel += acceleration * thrust;

        if velocity.linvel.length() > max_speed {
            velocity.linvel = velocity.linvel.normalize() * max_speed;
        }
    }
}

pub fn rotate_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_m: Query<(&mut Transform, &Rotatable)>,
    timer: Res<Time>,
) {
    for (mut transform, player_m) in &mut player_m {
        if keyboard.pressed(KeyCode::KeyJ) {
            transform.rotate_local_y(player_m.speed * 23.0 * timer.delta_secs());
        }
        if keyboard.pressed(KeyCode::KeyL) {
            transform.rotate_local_y(player_m.speed * -23.0 * timer.delta_secs());
        }
        if keyboard.pressed(KeyCode::KeyI) {
            transform.rotate_local_x(player_m.speed * 15.0 * timer.delta_secs());
        }
        if keyboard.pressed(KeyCode::KeyK) {
            transform.rotate_local_x(player_m.speed * -15.0 * timer.delta_secs());
        }
        if keyboard.pressed(KeyCode::KeyM) {
            transform.rotate_local_z(player_m.speed * 15.0 * timer.delta_secs());
        }
        if keyboard.pressed(KeyCode::KeyN) {
            transform.rotate_local_z(player_m.speed * -15.0 * timer.delta_secs());
        }
    }
}
