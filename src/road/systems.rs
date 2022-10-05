use super::components::*;
use crate::{car::components::CameraFocus, common::components::*};
use bevy::prelude::*;

pub(super) fn mid_lanes_system(
    mut cmd: Commands,
    q: Query<&Transform, With<MidLane>>,
    last_lane: Query<Entity, (With<MidLane>, With<Last>)>,
    focused: Query<&Transform, With<CameraFocus>>,
) {
    let entity = last_lane.single();
    let focused_y = focused.single().translation.y;

    if let Ok(transform) = q.get(entity) {
        if (focused_y.abs() - transform.translation.y.abs()).abs() < 600. {
            cmd.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(8., 50.)),
                    ..default()
                },
                transform: Transform::from_xyz(-33., transform.translation.y + 100., 0.),
                ..default()
            })
            .insert(MidLane)
            .insert(Last);

            cmd.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(8., 50.)),
                    ..default()
                },
                transform: Transform::from_xyz(33., transform.translation.y + 100., 0.),
                ..default()
            })
            .insert(MidLane);

            cmd.entity(entity).remove::<Last>();
        }
    }
}
