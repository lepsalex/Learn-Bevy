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
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AssetsPlugin)
            .add(InputPlugin)
            .add(CommonPlugin)
            .add(ProjectilePlugin)
    }
}
