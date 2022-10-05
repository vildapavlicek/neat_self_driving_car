pub mod components;
pub mod systems;

use crate::common::components::{CanCollide, Last};
use components::*;
use systems::*;

use bevy::prelude::{Commands, Plugin, *};

const LANE_SIZE: Vec2 = Vec2::new(10., 1024.);

pub struct RoadPlugin;

impl Plugin for RoadPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_road)
            .add_system(mid_lanes_system);
    }
}

fn spawn_road(mut cmd: Commands) {
    // left line
    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(LANE_SIZE),
            color: Color::WHITE,
            ..default()
        },
        transform: Transform::from_xyz(-100., 0., 0.),
        ..default()
    })
    .insert(Lane)
    .insert(CanCollide);

    // right line
    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(LANE_SIZE),
            color: Color::WHITE,
            ..default()
        },
        transform: Transform::from_xyz(100., 0., 0.),
        ..default()
    })
    .insert(Lane)
    .insert(CanCollide);

    spawn_mid_lines(&mut cmd);
}

fn spawn_mid_lines(cmd: &mut Commands) {
    let mut entity = None;
    for i in -3..7 {
        entity = Some(
            cmd.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(8., 50.)),
                    ..default()
                },
                transform: Transform::from_xyz(-33., i as f32 * 100., 0.),
                ..default()
            })
            .insert(MidLane)
            .id(),
        );

        cmd.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(8., 50.)),
                ..default()
            },
            transform: Transform::from_xyz(33., i as f32 * 100., 0.),
            ..default()
        })
        .insert(MidLane);
    }
    cmd.entity(entity.unwrap()).insert(Last);
}
