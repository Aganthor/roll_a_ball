use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod player;

use player::PLayerPlugin;

const WALL_THICKNESS: f32 = 0.5;
const WALL_COLLIDER_THICKNESS: f32 = WALL_THICKNESS / 2.0;
const WALL_COLOR: Color = Color::rgb(192.0, 192.0, 192.0);



fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Roll a ball".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            })
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PLayerPlugin)
        .add_systems(Startup, scene_setup)
        .run();
}

fn scene_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(10.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    })
    .insert(Collider::cuboid(10.0, 0.1, 10.0));

    // Spawn back wall
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(10.0, 1.0, WALL_THICKNESS).into()),
        material: materials.add(WALL_COLOR.into()),
        transform: Transform::from_xyz(0.0, 0.0, -5.0),
        ..default()
    })
    .insert(RigidBody::Fixed)
        .with_children(|children| {
            children.spawn(Collider::cuboid(5.0, 0.5, WALL_COLLIDER_THICKNESS));
        })
    .insert(Restitution::coefficient(0.7));

    // spawn front wall
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(10.0, 1.0, WALL_THICKNESS).into()),
        material: materials.add(WALL_COLOR.into()),
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..default()
    })
    .insert(RigidBody::Fixed)
        .with_children(|children| {
            children.spawn(Collider::cuboid(5.0, 0.5, WALL_COLLIDER_THICKNESS));
        })
    .insert(Restitution::coefficient(0.7));

    let mut t = Transform::from_translation(Vec3::new(-5.0, 0.0, 0.0));
    t.rotate_local_axis(Vec3::Y, 1.571);
        
    // spawn left wall (relative to camera)
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(10.0, 1.0, WALL_THICKNESS).into()),
        material: materials.add(WALL_COLOR.into()),
        transform: t,
        ..default()
    })
    .insert(RigidBody::Fixed)
        .with_children(|children| {
            children.spawn(Collider::cuboid(5.0, 0.5, WALL_COLLIDER_THICKNESS));
        })
    .insert(Restitution::coefficient(0.7));

    // spawn right wall (relative to camera)
    t = Transform::from_translation(Vec3::new(5.0, 0.0, 0.0));
    t.rotate_local_axis(Vec3::Y, 1.571);

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(10.0, 1.0, WALL_THICKNESS).into()),
        material: materials.add(WALL_COLOR.into()),
        transform: t,
        ..default()
    })
    .insert(RigidBody::Fixed)
    .with_children(|children| {
        children.spawn(Collider::cuboid(5.0, 0.5, WALL_COLLIDER_THICKNESS));
    })
    .insert(Restitution::coefficient(0.7));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 4.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}


