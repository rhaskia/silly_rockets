use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct PlayerMovementData {
    is_grounded: bool,
}

impl PlayerMovementData { fn new() -> PlayerMovementData { PlayerMovementData { is_grounded: false } } }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(48.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(player_movement)
        .add_system(display_events)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

fn display_events(mut collision_events: EventReader<CollisionEvent>, mut player: Query<(Entity, &mut PlayerMovementData)>) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(a, b, flags) => {
                let mut player = player.single_mut();
                if a == &player.0 || b == &player.0 { player.1.is_grounded = true; }
            },
            CollisionEvent::Stopped(a, b, flags) => {
                let mut player = player.single_mut();
                if a == &player.0 || b == &player.0 { player.1.is_grounded = false; }
            },
        }
    }
}

fn player_movement(mut player_q: Query<(&mut ExternalImpulse, &mut PlayerMovementData)>, keys: Res<Input<KeyCode>>,) {
    let (mut jump_force, mut player_data) = player_q.single_mut();

    if keys.just_pressed(KeyCode::Space) { manage_jumping(&mut jump_force, &mut player_data); }
    //if keys. { manage_jumping(&jump_force, &player_data); }
}

fn manage_jumping(mut force: &mut Mut<ExternalImpulse>, player_data: &mut Mut<PlayerMovementData>) {
    if !player_data.is_grounded { return; }

    force.impulse = Vec2::new(0.0, 200.0);
    force.torque_impulse = 0.0;
    player_data.is_grounded = false;
}

fn move_horizontal(direction: bool) {

}

fn setup_physics(mut commands: Commands, asset_server: Res<AssetServer>) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    let sprite = Sprite { custom_size: Some(Vect {x: 32., y: 64.}), ..default() };

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::capsule_y(16., 16.))
        .insert(Restitution::coefficient(0.1))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ExternalImpulse { impulse: Vec2::new(0.0, 0.0), torque_impulse: 0.0, })
        .insert(PlayerMovementData::new())
        .insert(SpriteBundle { texture: asset_server.load("sprites/player.png"), sprite, ..default() })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}