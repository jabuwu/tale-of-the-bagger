use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};

use crate::{AppState, AssetLibrary};

use super::{ConveyorPlugin, DeskPlugin, DeskSpawnEvent};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DeskPlugin)
            .add_plugin(ConveyorPlugin)
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(game_enter));
    }
}

fn game_enter(
    mut commands: Commands,
    mut desk_spawn_events: EventWriter<DeskSpawnEvent>,
    asset_library: Res<AssetLibrary>,
    audio: Res<Audio>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(SpriteBundle {
        texture: asset_library.textures.background.clone(),
        transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(0.75)),
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: asset_library.textures.background_front.clone(),
        transform: Transform::from_xyz(0., 0., 0.1).with_scale(Vec3::splat(0.75)),
        ..Default::default()
    });

    audio.play(asset_library.audio.ambience.clone()).looped();
    audio
        .play(asset_library.audio.radio_tune_1.clone())
        .looped();

    desk_spawn_events.send_default();
}
