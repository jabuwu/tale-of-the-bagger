use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{common::VersionSpawnEvent, AppState, AssetLibrary};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Loading).with_system(loading_enter))
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(loading_update));
    }
}

fn loading_enter(
    mut asset_library: ResMut<AssetLibrary>,
    mut skeletons: ResMut<Assets<SkeletonData>>,
    mut version_spawn_events: EventWriter<VersionSpawnEvent>,
    asset_server: Res<AssetServer>,
) {
    asset_library.load_assets(skeletons.as_mut(), asset_server.as_ref());
    version_spawn_events.send_default();
}

fn loading_update(mut app_state: ResMut<State<AppState>>, time: Res<Time>) {
    if time.seconds_since_startup() > 0.2 {
        let _ = app_state.set(AppState::Game);
    }
}
