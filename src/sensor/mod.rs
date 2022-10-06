pub mod components;
pub mod systems;
use systems::*;

use bevy::prelude::Plugin;

pub struct SensorPlugin;

impl Plugin for SensorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(debug_sensor);
    }
}
