use bevy::prelude::*;
use bevy::render::texture::ImageSampler;
use bevy_rapier2d::prelude::*;
mod player_movement;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(48.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, player_movement::movement)
        .add_systems(Update, player_movement::collision)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands, asset_server: Res<AssetServer>, mut assets: ResMut<Assets<Image>>,) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    let sprite = Sprite { custom_size: Some(Vect {x: 24., y: 48.}), ..default() };
    let texture = asset_server.load("sprites/player.png");

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::capsule_y(12., 12.))
        .insert(Restitution::coefficient(0.1))
        .insert(LockedAxes::ROTATION_LOCKED_Z)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(GravityScale(1.0))
        .insert(Damping { linear_damping: 0.5, angular_damping: 1.0 })

        .insert(ExternalImpulse { impulse: Default::default(), torque_impulse: 0.0 })
        .insert(Velocity { linvel: Default::default(), angvel: 0.0 })
        
        .insert(player_movement::Data::new(150., 200.))
        .insert(SpriteBundle { texture, sprite, ..default() })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}
