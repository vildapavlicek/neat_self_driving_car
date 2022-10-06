use bevy::prelude::*;

pub mod components;
use components::*;
pub mod systems;
use systems::*;

use crate::{
    common::components::CanCollide,
    sensor::components::{Sensor, SensorDetails},
};

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

    /* let s1 = cmd
    .spawn_bundle(crate::sensor::components::Sensor::new(
        0.,
        0.,
        crate::sensor::components::SensorDetails {
            angle: 0.,
            length: 120.,
        },
    ))
    .id(); */

    let s1 = cmd
        .spawn()
        .insert_bundle(TransformBundle::default())
        .insert(SensorDetails {
            angle: 0.,
            length: 80.,
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
    .add_child(s1);

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
