use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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

pub struct PLayerPlugin;

impl Plugin for PLayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, apply_force.after(player_input));

    }
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
    .insert(Velocity::linear(Vec3::ZERO))
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
    mut player_query: Query<(&mut Velocity, &Player)>,
    time: Res<Time>,
) {
    let (mut velocity, player) = player_query.single_mut();

    velocity.linvel += player.direction * player.speed * time.delta_seconds();
}

