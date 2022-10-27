use bevy::{app::PluginGroupBuilder, prelude::*};

use super::{CursorPlugin, ForceCameraRatioPlugin, SpineSync2Plugin, Transform2Plugin};

pub struct CommonPlugins;

impl PluginGroup for CommonPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(CursorPlugin);
        group.add(Transform2Plugin);
        group.add(SpineSync2Plugin::default());
        group.add(ForceCameraRatioPlugin);
    }
}
