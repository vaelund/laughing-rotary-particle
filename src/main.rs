//

// This lint usually gives bad advice in the context of Bevy -- hiding complex queries behind
// type aliases tends to obfuscate code while offering no improvement in code cleanliness.
#![allow(clippy::type_complexity)]

//! Demonstrates rotating entities in 2D using quaternions.

mod enemy;
mod player;

use crate::enemy::{RotateToPlayer, SnapToPlayer};
use crate::player::Player;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

#[derive(Resource)]
pub struct Bounds {
    pub max: Vec2,
}
const DEFAULT_BOUNDS: Bounds = Bounds {
    max: Vec2::new(1200.0, 640.0),
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            player::PlayerPlugin,
            enemy::EnemyBehaviorPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(DEFAULT_BOUNDS)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup_physics)
        .run();
}

/// Add the game's entities to our world and creates an orthographic camera for 2D rendering.
///
/// The Bevy coordinate system is the same for 2D and 3D, in terms of 2D this means that:
///
/// * `X` axis goes from left to right (`+X` points right)
/// * `Y` axis goes from bottom to top (`+Y` point up)
/// * `Z` axis goes from far to near (`+Z` points towards you, out of the screen)
///
/// The origin is at the center of the screen.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    bounds: Res<Bounds>,
) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());

    let horizontal_margin = bounds.max.x / 4.0;
    let vertical_margin = bounds.max.y / 4.0;

    // player controlled ship
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(10., 20.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
            ..default()
        },
        Player {
            movement_impulse: 10.0, // meters per second ^ 2
            rotation_impulse: 0.01, // radians per second ^ 2
        },
        RigidBody::Dynamic,
        Collider::cuboid(5.0, 10.0),
        Damping {
            linear_damping: 0.5,
            angular_damping: 10.0,
        },
        ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        },
    ));

    // enemy that snaps to face the player spawns on the bottom and left
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BEIGE,
                custom_size: Some(Vec2::new(10.0, 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0 - horizontal_margin, 0.0, 0.0),
            ..default()
        },
        SnapToPlayer,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BEIGE,
                custom_size: Some(Vec2::new(10.0, 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0 - vertical_margin, 0.0),
            ..default()
        },
        SnapToPlayer,
    ));

    // enemy that rotates to face the player enemy spawns on the top and right
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BISQUE,
                custom_size: Some(Vec2::new(10.0, 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0 + horizontal_margin, 0.0, 0.0),
            ..default()
        },
        RotateToPlayer {
            rotation_speed: f32::to_radians(45.0), // degrees per second
        },
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BISQUE,
                custom_size: Some(Vec2::new(10.0, 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0 + vertical_margin, 0.0),
            ..default()
        },
        RotateToPlayer {
            rotation_speed: f32::to_radians(90.0), // degrees per second
        },
    ));
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -200.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}
