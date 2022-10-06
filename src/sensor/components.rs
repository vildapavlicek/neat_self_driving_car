use bevy::prelude::{Bundle, Component, Transform};

#[derive(Component)]
pub struct SensorDetails {
    pub length: f32,
    pub angle: f32,
}

#[derive(Bundle)]
pub struct Sensor {
    details: SensorDetails,
    start: Transform,
}

impl Sensor {
    pub fn new(x: f32, y: f32, details: SensorDetails) -> Sensor {
        let start = Transform::from_xyz(x, y, 0.);

        Sensor { start, details }
    }
}
