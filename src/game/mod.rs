use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{assets::AssetsPlugin, common::CommonPlugin, projectile::ProjectilePlugin};

pub mod assets;
pub mod common;
pub mod projectile;
pub mod physics;

pub struct DefaultGamePlugins;
impl PluginGroup for DefaultGamePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(AssetsPlugin);
        group.add(CommonPlugin);
        group.add(ProjectilePlugin);
    }
}
