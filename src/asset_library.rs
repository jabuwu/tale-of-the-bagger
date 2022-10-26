use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_spine::prelude::*;

use crate::common::AssetCollection;

#[derive(Default)]
pub struct AssetLibrary {
    pub textures: TextureAssets,
    pub spines: SpineAssets,
    pub audio: AudioAssets,
}

impl AssetLibrary {
    pub fn load_assets(
        &mut self,
        skeletons: &mut Assets<SkeletonData>,
        asset_server: &AssetServer,
    ) {
        self.textures.load_assets(skeletons, asset_server);
        self.spines.load_assets(skeletons, asset_server);
        self.audio.load_assets(skeletons, asset_server);
    }
}

#[derive(Default, AssetCollection)]
pub struct TextureAssets {
    #[asset("textures/background.png")]
    pub background: Handle<Image>,
    #[asset("textures/background_front.png")]
    pub background_front: Handle<Image>,
}

#[derive(Default, AssetCollection)]
pub struct SpineAssets {
    #[spine_asset("spines/desk")]
    pub desk: Handle<SkeletonData>,
}

#[derive(Default, AssetCollection)]
pub struct AudioAssets {
    #[asset("audio/ambience.ogg")]
    pub ambience: Handle<AudioSource>,
    #[asset("audio/radio_tune_1.ogg")]
    pub radio_tune_1: Handle<AudioSource>,
}
