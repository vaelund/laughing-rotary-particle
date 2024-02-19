//

// This lint usually gives bad advice in the context of Bevy -- hiding complex queries behind
// type aliases tends to obfuscate code while offering no improvement in code cleanliness.
#![allow(clippy::type_complexity)]

mod enemy;
use crate::enemy::{RotateToPlayer, SnapToPlayer};
mod player;
use crate::player::Player;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
mod level;
use level::load_level_geo;

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
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            player::PlayerPlugin,
            enemy::EnemyBehaviorPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            //RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(DEFAULT_BOUNDS)
        .add_systems(Startup, setup)
        .add_systems(Startup, load_level_geo)
        .add_systems(Update, bevy::window::close_on_esc)
        //.add_systems(Startup, setup_physics_demo)
        .run();
}

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
            transform: Transform::from_xyz(200.0, 200.0, 0.0),
            ..default()
        },
        Player {
            movement_impulse: 10.0,
            rotation_impulse: 0.01,
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
        Ccd::enabled(),
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
