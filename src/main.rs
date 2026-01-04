#![allow(unused_variables)]

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

mod components;
mod systems;

use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(EguiPlugin::default())
        // .add_plugins(WorldInspectorPlugin::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(ClearColor(Color::srgb(0.03, 0.03, 0.15)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_player,
                follow_player,
                rotate_player,
                camera_mouse_control,
            ),
        )
        .run();
}
