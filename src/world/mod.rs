use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{camera::CameraPlugin, level::LevelPlugin, lighting::LightingPlugin};

pub mod camera;
pub mod level;
pub mod lighting;

pub struct DefaultWorldPlugins;
impl PluginGroup for DefaultWorldPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(LevelPlugin);
        group.add(LightingPlugin);
        group.add(CameraPlugin);
    }
}
