use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use lerp::Lerp;
use rand::{random, seq::SliceRandom, thread_rng};

use crate::{AppState, AssetLibrary};

pub struct AmbiencePlugin;

impl Plugin for AmbiencePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ambience_update);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmbienceKind {
    Song,
    DingDong,
    Announcement,
}

pub struct AmbienceUpdateLocal {
    first: bool,
    wait_time: f32,
    last_ambience: AmbienceKind,
}

impl Default for AmbienceUpdateLocal {
    fn default() -> Self {
        Self {
            first: true,
            wait_time: 5.,
            last_ambience: AmbienceKind::Announcement,
        }
    }
}

fn ambience_update(
    mut local: Local<AmbienceUpdateLocal>,
    time: Res<Time>,
    asset_library: Res<AssetLibrary>,
    audio: Res<Audio>,
    state: Res<State<AppState>>,
) {
    if local.first && *state.current() != AppState::Loading {
        audio.play(asset_library.audio.ambience.clone()).looped();
        local.first = false;
    }
    if *state.current() == AppState::Game {
        local.wait_time -= time.delta_seconds();
    }
    if local.wait_time <= 0. {
        let mut rng = thread_rng();
        if local.last_ambience == AmbienceKind::DingDong {
            audio.play(
                [
                    asset_library.audio.announcement_1.clone(),
                    asset_library.audio.announcement_2.clone(),
                    asset_library.audio.announcement_3.clone(),
                    asset_library.audio.announcement_4.clone(),
                    asset_library.audio.announcement_5.clone(),
                    asset_library.audio.announcement_6.clone(),
                    asset_library.audio.announcement_7.clone(),
                    asset_library.audio.announcement_8.clone(),
                ]
                .choose(&mut rng)
                .unwrap()
                .clone(),
            );
            local.last_ambience = AmbienceKind::Announcement;
            local.wait_time = 14.;
        } else {
            let ambience = if local.last_ambience == AmbienceKind::Song {
                AmbienceKind::DingDong
            } else {
                *[AmbienceKind::Song, AmbienceKind::DingDong]
                    .choose(&mut rng)
                    .unwrap()
            };
            match ambience {
                AmbienceKind::Song => match [0, 1, 2].choose(&mut rng).unwrap() {
                    0 => {
                        audio.play(asset_library.audio.radio_tune_1.clone());
                        local.wait_time = 30.;
                    }
                    1 => {
                        audio.play(asset_library.audio.radio_tune_2.clone());
                        local.wait_time = 60.;
                    }
                    2 => {
                        audio.play(asset_library.audio.radio_tune_3.clone());
                        local.wait_time = 60.;
                    }
                    _ => unreachable!(),
                },
                AmbienceKind::DingDong => {
                    audio.play(asset_library.audio.ding_dong.clone());
                    local.wait_time = 3.5;
                }
                _ => {
                    unreachable!();
                }
            }
            local.last_ambience = ambience;
        }
        if local.last_ambience != AmbienceKind::DingDong {
            local.wait_time += 3.0_f32.lerp(30., random::<f32>());
        }
    }
}
