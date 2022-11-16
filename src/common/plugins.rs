use bevy::{app::PluginGroupBuilder, prelude::*};

use super::{
    ClearScenePlugin, ForceCameraRatioPlugin, GameInputPlugin, InteractablePlugin,
    SpineSync2Plugin, Transform2Plugin, VersionPlugin,
};

pub struct CommonPlugins;

impl PluginGroup for CommonPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group.add(Transform2Plugin);
        group = group.add(SpineSync2Plugin::default());
        group = group.add(ForceCameraRatioPlugin);
        group = group.add(InteractablePlugin);
        group = group.add(GameInputPlugin);
        group = group.add(VersionPlugin);
        group = group.add(ClearScenePlugin);
        group
    }
}
