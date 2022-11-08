use bevy::{app::PluginGroupBuilder, prelude::*};

use super::{
    ClearScenePlugin, ForceCameraRatioPlugin, GameInputPlugin, InteractablePlugin,
    SpineSync2Plugin, Transform2Plugin, VersionPlugin,
};

pub struct CommonPlugins;

impl PluginGroup for CommonPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(Transform2Plugin);
        group.add(SpineSync2Plugin::default());
        group.add(ForceCameraRatioPlugin);
        group.add(InteractablePlugin);
        group.add(GameInputPlugin);
        group.add(VersionPlugin);
        group.add(ClearScenePlugin);
    }
}
