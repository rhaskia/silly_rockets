use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Data {
    is_grounded: bool,
    speed: f32,
    max_speed: f32,
}

impl Data {
    pub fn new(speed: f32, max_speed: f32) -> Data {
        Data {
            is_grounded: false,
            speed,
            max_speed
        }
    }
}

pub fn collision(
    mut collision_events: EventReader<CollisionEvent>, mut player: Query<(Entity, &mut Data)>) {
    for collision_event in collision_events.iter() {
        //TODO clean
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

pub fn movement(mut player_q: Query<(&mut ExternalImpulse, &mut Velocity, &mut Data)>, keys: Res<Input<KeyCode>>,) {
    let (mut jump_force,
        mut velocity,
        mut player_data) = player_q.single_mut();

    if keys.just_pressed(KeyCode::Space) { jump(&mut jump_force, &mut player_data); }

    if keys.pressed(KeyCode::D) {
        velocity.linvel += Vec2 { x: player_data.speed, y: 0. };
        if velocity.linvel.x > player_data.max_speed { velocity.linvel.x = player_data.max_speed; }
    }

    if keys.pressed(KeyCode::A) {
        velocity.linvel -= Vec2 { x: player_data.speed, y: 0. };
        if velocity.linvel.x < -player_data.max_speed { velocity.linvel.x = -player_data.max_speed; }
    }
}

pub fn jump(mut force: &mut Mut<ExternalImpulse>, player_data: &mut Mut<Data>) {
    if !player_data.is_grounded { return; }

    force.impulse = Vec2::new(0.0, 30.0);
    force.torque_impulse = 0.0;
    player_data.is_grounded = false;
}
