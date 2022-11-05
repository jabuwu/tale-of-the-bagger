use bevy::{app::PluginGroupBuilder, prelude::*};

use super::{ProductJerkyPlugin, ProductKetchupPlugin};

pub struct ProductPlugins;

impl PluginGroup for ProductPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(ProductJerkyPlugin);
        group.add(ProductKetchupPlugin);
    }
}
