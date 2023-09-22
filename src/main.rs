use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const WALL_THICKNESS: f32 = 0.5;
const WALL_COLLIDER_THICKNESS: f32 = WALL_THICKNESS / 2.0;
const WALL_COLOR: Color = Color::rgb(192.0, 192.0, 192.0);

#[derive(Component)]
struct Player {
    direction: Vec3,
    speed: f32,
}

impl Player {
    fn new() -> Self {
        Player {
            direction: Vec3::ZERO,
            speed: 5.0,
        }
    }
}

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
        .add_systems(Startup, scene_setup)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, apply_force.after(player_input))
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

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>, 
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: 0.5,
                subdivisions: 3,
            })
            .unwrap(),
        ),
        material: materials.add(StandardMaterial { 
            base_color: Color::rgb(1.0, 0.0, 0.0), 
            metallic: 0.5,
            ..default()
        }),
        transform: Transform {
            translation: Vec3 { x: 0.0, y: 0.25, z: 0.0 },
            scale: Vec3 { x: 0.5, y: 0.5, z: 0.5 },
            ..default()
        },
        ..default()
    })
    .insert(Player::new())
    .insert(RigidBody::Dynamic)
        .with_children(|children| {
            children.spawn(Collider::ball(0.25));
        })
    .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::DYNAMIC_STATIC)
    .insert(Restitution::coefficient(0.7))
    .insert(ColliderMassProperties::Density(2.0));
}

fn player_input(
    mut player_query: Query<&mut Player>,
    input: Res<Input<KeyCode>>,
) {
    for mut player in player_query.iter_mut() {
        if input.pressed(KeyCode::A) {
            player.direction = Vec3::NEG_X;
        }
        if input.pressed(KeyCode::D) {
            player.direction = Vec3::X;
        }
        if input.pressed(KeyCode::S) {
            player.direction = Vec3::Z;
        }
        if input.pressed(KeyCode::W) {
            player.direction = Vec3::NEG_Z;        
        }        
    }
}

fn apply_force(
    mut player_query: Query<(&Player, &RigidBody, &Collider)>,
) {

}
