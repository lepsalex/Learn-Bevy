use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{
    assets::AssetsPlugin, common::CommonPlugin, input::InputPlugin, projectile::ProjectilePlugin,
};

pub mod assets;
pub mod common;
pub mod input;
pub mod physics;
pub mod projectile;

pub struct DefaultGamePlugins;
impl PluginGroup for DefaultGamePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(AssetsPlugin);
        group.add(InputPlugin);
        group.add(CommonPlugin);
        group.add(ProjectilePlugin);
    }
}
