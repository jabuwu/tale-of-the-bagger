use bevy::{app::PluginGroupBuilder, prelude::*};

use super::CursorPlugin;

pub struct CommonPlugins;

impl PluginGroup for CommonPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(CursorPlugin);
    }
}
