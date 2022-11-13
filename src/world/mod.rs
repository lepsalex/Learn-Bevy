use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{
    camera::CameraPlugin, level::LevelPlugin, lighting::LightingPlugin,
    navigation::NavigationPlugin, spawner::SpawnerPlugin,
};

pub mod camera;
pub mod level;
pub mod lighting;
pub mod navigation;
pub mod spawner;

pub struct DefaultWorldPlugins;
impl PluginGroup for DefaultWorldPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(LevelPlugin)
            .add(LightingPlugin)
            .add(CameraPlugin)
            .add(NavigationPlugin)
            .add(SpawnerPlugin)
    }
}
