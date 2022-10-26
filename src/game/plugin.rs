use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_spine::prelude::*;

use crate::{AppState, AssetLibrary};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(game_enter))
            .add_system(desk_spawned);
    }
}

fn game_enter(mut commands: Commands, asset_library: Res<AssetLibrary>, audio: Res<Audio>) {
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

    commands.spawn_bundle(SpineBundle {
        skeleton: asset_library.spines.desk.clone(),
        transform: Transform::from_xyz(-105., -256.5, 0.2).with_scale(Vec3::splat(0.75)),
        ..Default::default()
    });
}

fn desk_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut spine_query: Query<&mut Spine>,
) {
    for event in spine_ready_event.iter() {
        if let Ok(mut spine) = spine_query.get_mut(event.entity) {
            let _ = spine
                .animation_state
                .set_animation_by_name(0, "conveyor", true);
        }
    }
}
