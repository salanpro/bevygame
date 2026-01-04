use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Rotatable {
    pub speed: f32,
}

#[derive(Component)]
pub struct FollowCamera {
    pub distance: f32,
    pub height: f32,
    pub smoothness: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
}

impl Default for FollowCamera {
    fn default() -> Self {
        Self {
            distance: 10.0,
            height: 4.5,
            smoothness: 5.0,
            yaw: 0.0,
            pitch: 0.3,  
            sensitivity: 0.003,
        }
    }
}
