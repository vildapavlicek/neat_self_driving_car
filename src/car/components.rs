use bevy::prelude::Component;

/// This marks entity / sprite that camera should be focused on and follow
/// There should always be one and only entity that we have focused
#[derive(Component)]
pub struct CameraFocus;

#[derive(Component)]
pub struct Player;

/// Use to mark cars that collided (crashed) into / with something
/// Damaged cars should be stopped and removed as a 'dead' cars
#[derive(Component)]
pub struct Damaged;

#[derive(Component)]
pub struct Controls {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Controls {
    pub fn move_up() -> Self {
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
pub struct Speed {
    pub max_speed: f32,
    pub acceleration: f32,
    pub turn_angle: f32,
    pub friction: f32,
    pub speed: f32,
    pub angle: f32,
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
