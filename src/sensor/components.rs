use bevy::prelude::{Bundle, Component, Quat, Transform};
use std::ops::{Deref, DerefMut};

#[derive(Component)]
pub(super) struct Length(f32);

impl Deref for Length {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Length {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Bundle)]
pub struct Sensor {
    length: Length,
    transform: Transform,
}

impl Sensor {
    pub fn new(x: f32, y: f32, angle: f32, length: f32) -> Sensor {
        let rot = Quat::from_rotation_z(angle);
        let mut transform = Transform::from_xyz(x, y, 0.);
        transform.rotation = rot;
        Sensor {
            length: Length(length),
            transform: transform,
        }
    }
}
