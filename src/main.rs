use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
mod car;
mod common;
mod road;
mod sensor;

mod utils;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "NEAT self driving car".into(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(car::CarPlugin)
        .add_plugin(road::RoadPlugin)
        .add_plugin(sensor::SensorPlugin)
        .run();
}
