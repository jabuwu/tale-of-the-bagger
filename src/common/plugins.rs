use bevy::{app::PluginGroupBuilder, prelude::*};

use super::{
    ForceCameraRatioPlugin, GameInputPlugin, InteractablePlugin, SpineSync2Plugin, Transform2Plugin,
};

pub struct CommonPlugins;

impl PluginGroup for CommonPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(Transform2Plugin);
        group.add(SpineSync2Plugin::default());
        group.add(ForceCameraRatioPlugin);
        group.add(InteractablePlugin);
        group.add(GameInputPlugin);
    }
}