Basic template for movement, and camera that follows the player.

To watch the collision debugger and world inspector just uncomment the marked lines below on 
`src/main.rs`
```rust

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
```
The collider is set in 
`systems/setup.rs`
```rust
 .with_children(|parent| {
        parent.spawn((
            Collider::capsule_y(0.75, 1.0), //HERE IS WHAT YOU SEE IN DEBUGGER
            Transform::from_xyz(0.0, 0.0, 0.0)
                .with_scale(Vec3::new(1.5, 1.8, 1.5)),
            GlobalTransform::default(),
        ));
```

Demo

https://github.com/user-attachments/assets/29250c6d-a57c-4c01-b445-0702a826714a




Asset model from: https://skfb.ly/pEVKO
