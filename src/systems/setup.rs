use crate::components::{FollowCamera, Player, Rotatable};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    //COLORS
    let red_color = materials.add(Color::srgb_u8(255, 0, 0));
    let green_color = materials.add(Color::srgb_u8(0, 255, 0));
    let blue_color = materials.add(Color::srgb_u8(0, 0, 255));
    let yellow_color = materials.add(Color::srgb_u8(255, 255, 0));
    let magenta_color = materials.add(Color::srgb_u8(255, 0, 255));
    let cyan_color = materials.add(Color::srgb_u8(0, 255, 255));
    let orange_color = materials.add(Color::srgb_u8(255, 125, 0));
    let white_color = materials.add(Color::srgb_u8(255, 255, 255));

    //Meshes
    let plane_mesh = meshes.add(Plane3d::default().mesh().size(36.0, 36.0));
    let cube1 = meshes.add(Cuboid::new(2.0, 2.0, 2.0));
    let sphere1 = meshes.add(Sphere::new(0.2));

    commands
        .spawn((
            RigidBody::Dynamic,
            GravityScale(0.),
            Velocity::zero(),
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            LockedAxes::ROTATION_LOCKED,
            Transform::from_xyz(0.0, 2.7, 0.0).with_scale(Vec3::splat(0.6)),
            Rotatable { speed: 0.3 },
            GlobalTransform::default(),
            Player,
        ))
        .with_children(|parent| {
        parent.spawn((
            Collider::capsule_y(0.75, 1.0),
            Transform::from_xyz(0.0, 0.0, 0.0)
                .with_scale(Vec3::new(1.5, 1.8, 1.5)),
            GlobalTransform::default(),
        ));
        parent.spawn((
            SceneRoot(asset_server.load(
                GltfAssetLabel::Scene(0).from_asset("models/holly/scene.gltf"),
            )),
            Transform::from_xyz(0.0, -2.7, 0.0),  
        ));
    });
        
    commands.spawn((
        Mesh3d(cube1),
        MeshMaterial3d(green_color),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(1.0, 1.0, 1.0),
    ));

    commands.spawn((
        Mesh3d(plane_mesh),
        MeshMaterial3d(white_color),
        Transform::from_xyz(0., 0., 0.),
        RigidBody::Fixed,
        Collider::cuboid(18.0, 0.05, 18.),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    // ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        FollowCamera::default(),
    ));
}
