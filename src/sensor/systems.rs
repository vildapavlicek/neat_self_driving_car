use super::components::*;
use bevy::prelude::{Color, Query, ResMut, Transform, Vec3};
use bevy_prototype_debug_lines::DebugLines;

pub(super) fn debug_sensor(
    sensors: Query<(&Transform, &SensorDetails)>,
    mut debug_lines: ResMut<DebugLines>,
) {
    for (transform, sensor_details) in sensors.iter() {
        dbg!(transform.translation);
        let start = transform.translation;
        let end_y = start.y + (sensor_details.angle.sin() * sensor_details.length);
        let end_x = start.x + (sensor_details.angle.cos() * sensor_details.length);
        let end = Vec3::new(end_x, end_y, 0.);

        debug_lines.line_colored(start, end, 0., Color::CYAN);
    }
}
