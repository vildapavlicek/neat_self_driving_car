use super::*;
use crate::utils::*;
use std::ops::Neg;

pub(super) fn handle_key_input(
    key_inputs: ResMut<Input<KeyCode>>,
    mut controls: Query<&mut Controls, With<Player>>,
) {
    let mut controls = controls.single_mut();
    match (
        key_inputs.just_pressed(KeyCode::Up),
        key_inputs.just_released(KeyCode::Up),
    ) {
        (true, false) => controls.up = true,
        (false, true) => controls.up = false,
        _ => (),
    }

    match (
        key_inputs.just_pressed(KeyCode::Down),
        key_inputs.just_released(KeyCode::Down),
    ) {
        (true, false) => controls.down = true,
        (false, true) => controls.down = false,
        _ => (),
    }

    match (
        key_inputs.just_pressed(KeyCode::Left),
        key_inputs.just_released(KeyCode::Left),
    ) {
        (true, false) => controls.left = true,
        (false, true) => controls.left = false,
        _ => (),
    }

    match (
        key_inputs.just_pressed(KeyCode::Right),
        key_inputs.just_released(KeyCode::Right),
    ) {
        (true, false) => controls.right = true,
        (false, true) => controls.right = false,
        _ => (),
    }
}

pub(super) fn controls_to_speed(mut controls: Query<(&Controls, &mut Speed), Without<Damaged>>) {
    controls.iter_mut().for_each(|(controls, mut speed)| {
        speed.speed = f32::clamp(
            speed.speed
                + (bool_to_f32(controls.up) * speed.acceleration
                    - bool_to_f32(controls.down) * speed.acceleration),
            (speed.max_speed / 2.0).neg(),
            speed.max_speed,
        );

        speed.angle += speed.turn_angle
            * match (controls.left, controls.right) {
                (true, false) if speed.speed < 0. => -1.,
                (true, false) => 1.,
                (false, true) if speed.speed < 0. => 1.,
                (false, true) => -1.,
                _ => 0.,
            };

        match speed.speed > 0.0 {
            true => speed.speed -= speed.friction,
            false => speed.speed += speed.friction,
        }

        if speed.speed.abs() < speed.acceleration / 2. {
            speed.speed = 0.0;
        }
    });
}

pub(super) fn move_car(mut moveable: Query<(&mut Transform, &Speed), Without<Damaged>>) {
    // let (mut transform, speed) = moveable.single_mut();

    for (mut transform, speed) in moveable.iter_mut() {
        transform.translation.x += speed.speed * speed.angle.sin().neg();
        transform.translation.y += speed.speed * speed.angle.cos();

        transform.rotation = Quat::from_rotation_z(speed.angle)
    }
}

pub(super) fn sync_camera(
    mut set: ParamSet<(
        Query<&Transform, With<CameraFocus>>,
        Query<&mut Transform, With<Camera>>,
        Query<&mut Transform, With<crate::road::components::Lane>>,
    )>,
) {
    let (x, y) = (
        set.p0().single().translation.x,
        set.p0().single().translation.y,
    );

    set.p1().single_mut().translation.x = x;
    set.p1().single_mut().translation.y = y + 200.;

    set.p2()
        .iter_mut()
        .for_each(|mut transform| transform.translation.y = y)
}

pub(super) fn kill(
    mut cmd: Commands,
    active: Query<(Entity, &Transform), Without<Damaged>>,
    colliders: Query<(Entity, &Transform), With<CanCollide>>,
) {
    active.iter().for_each(|(entity, transform_a)| {
        if colliders
            .iter()
            .find(|(collider_entity, transform_b)| {
                bevy::sprite::collide_aabb::collide(
                    transform_a.translation,
                    CAR_SIZE,
                    transform_b.translation,
                    CAR_SIZE,
                )
                .is_some()
                    && entity != *collider_entity
            })
            .is_some()
        {
            cmd.entity(entity).insert(Damaged);
        }
    });
}

pub(super) fn stop_dead(mut q: Query<(Entity, &mut Speed), Added<Damaged>>) {
    for (entity, mut speed) in q.iter_mut() {
        error!(
            ?entity,
            "changed 'alive', removed 'alive' component from entity"
        );
        speed.speed = 0.0;
    }
}
