use bevy::prelude::*;
use bevy_rapier2d::dynamics::ExternalImpulse;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (player_movement_system,));
    }
}

/// player component
#[derive(Component)]
pub(crate) struct Player {
    /// linear speed in meters per second
    pub(crate) movement_impulse: f32,
    /// rotation speed in radians per second
    pub(crate) rotation_impulse: f32,
}

/// Demonstrates applying rotation and movement based on keyboard input.
fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut ExternalImpulse, &Transform)>,
) {
    let (player, mut external_impulse, transform) = query
        .get_single_mut()
        .expect("No or more than one Player entity found");
    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        movement_factor -= 1.0;
    }

    // rotation
    external_impulse.torque_impulse =
        rotation_factor * player.rotation_impulse * time.delta_seconds();

    // calculate the linear impulse based on the rotation of the ship

    let magnitude = movement_factor * player.movement_impulse * time.delta_seconds();
    let direction = transform.rotation.mul_vec3(Vec3::Y);

    external_impulse.impulse = Vec2::new(direction.x * magnitude, direction.y * magnitude);
}
