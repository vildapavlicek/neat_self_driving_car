use bevy::prelude::*;

pub mod components;
use components::*;
pub mod systems;
use systems::*;

use crate::common::components::CanCollide;

const CAR_SIZE: Vec2 = Vec2::new(25., 50.);
const CAR_Z_LAYER: f32 = 1.0;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(start_up)
            .add_system(handle_key_input)
            .add_system(controls_to_speed)
            .add_system(move_car)
            .add_system(sync_camera)
            .add_system(kill)
            .add_system(stop_dead);
    }
}

fn start_up(mut cmd: Commands) {
    cmd.spawn_bundle(Camera2dBundle::default());

    // START SECTION
    // DBG lines

    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2 { x: 1., y: 80. }),
            color: Color::YELLOW,
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 2.),
        ..default()
    });

    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2 { x: 80., y: 1. }),
            color: Color::YELLOW,
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 2.),
        ..default()
    });

    // END SECTION

    // test child
    let id = cmd
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 1., y: 80. }),
                color: Color::GREEN,
                ..default()
            },
            transform: Transform::from_xyz(0., 65., 2.),
            ..default()
        })
        .id();

    ///// ********************
    let x = 0.78f32.cos() * 40.;
    let y = 0.78f32.sin() * 40.;

    let mut transform = Transform::from_xyz(x + 12.5, y + 25.0, 2.);
    transform.rotation = Quat::from_rotation_z(-0.7853981634);

    let id_2 = cmd
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 1., y: 80. }),
                color: Color::CYAN,
                ..default()
            },
            transform: transform,
            ..default()
        })
        .id();

    // player car
    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(CAR_SIZE),
            color: Color::RED,
            ..default()
        },
        transform: Transform::from_xyz(0., 0., CAR_Z_LAYER),
        ..default()
    })
    .insert(Speed::default())
    .insert(Controls::default())
    .insert(CameraFocus)
    .insert(Player)
    .add_child(id)
    .add_child(id_2);

    spawn_obstacles(&mut cmd);
}

fn spawn_obstacles(cmd: &mut Commands) {
    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(CAR_SIZE),
            color: Color::YELLOW_GREEN,
            ..default()
        },
        transform: Transform::from_xyz(-66., 0., CAR_Z_LAYER),
        ..default()
    })
    .insert(CanCollide)
    .insert(Speed {
        max_speed: 4.0,
        ..default()
    })
    .insert(Controls::move_up());

    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(CAR_SIZE),
            color: Color::YELLOW_GREEN,
            ..default()
        },
        transform: Transform::from_xyz(0., 250., CAR_Z_LAYER),
        ..default()
    })
    .insert(CanCollide)
    .insert(Speed {
        max_speed: 4.0,
        ..default()
    })
    .insert(Controls::move_up());

    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(CAR_SIZE),
            color: Color::YELLOW_GREEN,
            ..default()
        },
        transform: Transform::from_xyz(66., 0., CAR_Z_LAYER),
        ..default()
    })
    .insert(CanCollide)
    .insert(Speed {
        max_speed: 4.0,
        ..default()
    })
    .insert(Controls::move_up());
}
