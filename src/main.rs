use bevy::prelude::*;

#[derive(Component)]
struct PlayerData {
    name: String
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteBundle {
        texture: asset_server.load("sprites/player.png"),
        ..default()
    },
    PlayerData { name: String::from("william") }
    ));
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