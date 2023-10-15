use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct PlayerData {
    name: String
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, print_ball_altitude)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(RigidBody::Dynamic)
        .insert(SpriteBundle {
            texture: asset_server.load("sprites/player.png"),
            ..default()
        })
        .insert(Velocity {
            linvel: Vec2::new(1.0, 2.0),
            angvel: 0.2
        })
        .insert(GravityScale(0.5))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled());
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

fn player_movement(
    mut player_q: Query<(&mut Transform, &mut PlayerData)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut trans, mut data) in &mut player_q {
        if keys.pressed(KeyCode::W) {
        trans.translation.x += 30.* time.delta_seconds();
        }
    }
}