use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use std::ops::{Deref, Neg};

mod car;
mod common;
mod road;
mod sensor;

mod utils;
use utils::bool_to_f32;

const CAR_SIZE: Vec2 = Vec2::new(25., 50.);
const CAR_Z_LAYER: f32 = 1.0;
const LINE_SIZE: Vec2 = Vec2::new(10., 1024.);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "NEAT self driving car".into(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(start_up)
        .add_system(handle_key_input)
        .add_system(controls_to_speed)
        .add_system(move_car)
        .add_system(sync_camera)
        .add_system(kill)
        .add_system(stop_dead)
        .add_system(mid_lines_system)
        .run();
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

    // left line
    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(LINE_SIZE),
            color: Color::WHITE,
            ..default()
        },
        transform: Transform::from_xyz(-100., 0., 0.),
        ..default()
    })
    .insert(Line)
    .insert(CanCollide);

    // right line
    cmd.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(LINE_SIZE),
            color: Color::WHITE,
            ..default()
        },
        transform: Transform::from_xyz(100., 0., 0.),
        ..default()
    })
    .insert(Line)
    .insert(CanCollide);

    spawn_mid_lines(&mut cmd);
    spawn_obstacles(&mut cmd);
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

#[derive(Component)]
struct CameraFocus;

#[derive(Component)]
struct Controls {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Controls {
    fn move_up() -> Self {
        Controls {
            up: true,
            down: false,
            left: false,
            right: false,
        }
    }
}

impl Default for Controls {
    fn default() -> Self {
        Controls {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

#[derive(Component)]
struct Speed {
    max_speed: f32,
    acceleration: f32,
    turn_angle: f32,
    friction: f32,
    speed: f32,
    angle: f32,
}

impl Default for Speed {
    fn default() -> Self {
        Speed {
            max_speed: 5.0,
            acceleration: 0.3,
            turn_angle: 0.01,
            friction: 0.1,
            speed: 0.0,
            angle: 0.0,
        }
    }
}

#[derive(Component)]
pub struct MidLane;

#[derive(Component)]
pub struct Last;

#[derive(Component)]
pub struct CanCollide;

#[derive(Component)]
pub struct Damaged;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Line;

#[derive(Component)]
pub struct Length(f32);

fn handle_key_input(
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

fn controls_to_speed(mut controls: Query<(&Controls, &mut Speed), Without<Damaged>>) {
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

fn move_car(mut moveable: Query<(&mut Transform, &Speed), Without<Damaged>>) {
    // let (mut transform, speed) = moveable.single_mut();

    for (mut transform, speed) in moveable.iter_mut() {
        transform.translation.x += speed.speed * speed.angle.sin().neg();
        transform.translation.y += speed.speed * speed.angle.cos();

        transform.rotation = Quat::from_rotation_z(speed.angle)
    }
}

fn sync_camera(
    mut set: ParamSet<(
        Query<&Transform, With<CameraFocus>>,
        Query<&mut Transform, With<Camera>>,
        Query<&mut Transform, With<Line>>,
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

fn kill(
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

fn stop_dead(mut q: Query<(Entity, &mut Speed), Added<Damaged>>) {
    for (entity, mut speed) in q.iter_mut() {
        error!(
            ?entity,
            "changed 'alive', removed 'alive' component from entity"
        );
        speed.speed = 0.0;
    }
}

struct MidLaneEntityBuffer(Entity);
impl Deref for MidLaneEntityBuffer {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn mid_lines_system(
    mut cmd: Commands,
    q: Query<&Transform, With<MidLane>>,
    entity: Query<Entity, (With<MidLane>, With<Last>)>,
    focused: Query<&Transform, With<CameraFocus>>,
) {
    let entity = entity.single();
    let focused_y = focused.single().translation.y;

    if let Ok(transform) = q.get(entity) {
        if (focused_y.abs() - transform.translation.y.abs()).abs() < 600. {
            info!("adding new set of lines");
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
