use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_spine::prelude::*;

use crate::common::AssetCollection;

#[derive(Default)]
pub struct AssetLibrary {
    pub textures: TextureAssets,
    pub fonts: FontAssets,
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
        self.fonts.load_assets(skeletons, asset_server);
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

    #[asset("textures/icons/meat.png")]
    pub icon_meat: Handle<Image>,
}

#[derive(Default, AssetCollection)]
pub struct FontAssets {
    #[asset("fonts/FiraSans-Bold.ttf")]
    pub default: Handle<Font>,
}

#[derive(Default, AssetCollection)]
pub struct SpineAssets {
    #[spine_asset("spines/desk")]
    pub desk: Handle<SkeletonData>,
    #[spine_asset("spines/bag")]
    pub bag: Handle<SkeletonData>,
    #[spine_asset("spines/customer")]
    pub customer: Handle<SkeletonData>,

    #[spine_asset("spines/products/jerky")]
    pub product_jerky: Handle<SkeletonData>,
    #[spine_asset("spines/products/ketchup")]
    pub product_ketchup: Handle<SkeletonData>,
}

#[derive(Default, AssetCollection)]
pub struct AudioAssets {
    #[asset("audio/ambience.ogg")]
    pub ambience: Handle<AudioSource>,

    #[asset("audio/radio_tune_1.ogg")]
    pub radio_tune_1: Handle<AudioSource>,
    #[asset("audio/radio_tune_2.ogg")]
    pub radio_tune_2: Handle<AudioSource>,
    #[asset("audio/radio_tune_3.ogg")]
    pub radio_tune_3: Handle<AudioSource>,

    #[asset("audio/ding_dong.ogg")]
    pub ding_dong: Handle<AudioSource>,

    #[asset("audio/announcement_1.ogg")]
    pub announcement_1: Handle<AudioSource>,
    #[asset("audio/announcement_2.ogg")]
    pub announcement_2: Handle<AudioSource>,
    #[asset("audio/announcement_3.ogg")]
    pub announcement_3: Handle<AudioSource>,
    #[asset("audio/announcement_4.ogg")]
    pub announcement_4: Handle<AudioSource>,
    #[asset("audio/announcement_5.ogg")]
    pub announcement_5: Handle<AudioSource>,
    #[asset("audio/announcement_6.ogg")]
    pub announcement_6: Handle<AudioSource>,
    #[asset("audio/announcement_7.ogg")]
    pub announcement_7: Handle<AudioSource>,
    #[asset("audio/announcement_8.ogg")]
    pub announcement_8: Handle<AudioSource>,

    #[asset("audio/bag_insert_1.ogg")]
    pub bag_insert_1: Handle<AudioSource>,
    #[asset("audio/bag_insert_2.ogg")]
    pub bag_insert_2: Handle<AudioSource>,
    #[asset("audio/bag_insert_3.ogg")]
    pub bag_insert_3: Handle<AudioSource>,
    #[asset("audio/bag_insert_4.ogg")]
    pub bag_insert_4: Handle<AudioSource>,
    #[asset("audio/bag_insert_5.ogg")]
    pub bag_insert_5: Handle<AudioSource>,
    #[asset("audio/bag_insert_6.ogg")]
    pub bag_insert_6: Handle<AudioSource>,
    #[asset("audio/bag_insert_7.ogg")]
    pub bag_insert_7: Handle<AudioSource>,
    #[asset("audio/bag_insert_8.ogg")]
    pub bag_insert_8: Handle<AudioSource>,
    #[asset("audio/bag_insert_9.ogg")]
    pub bag_insert_9: Handle<AudioSource>,
    #[asset("audio/bag_insert_10.ogg")]
    pub bag_insert_10: Handle<AudioSource>,
    #[asset("audio/bag_insert_11.ogg")]
    pub bag_insert_11: Handle<AudioSource>,

    #[asset("audio/bag_clear_success_1.ogg")]
    pub bag_clear_success_1: Handle<AudioSource>,
    #[asset("audio/bag_clear_success_2.ogg")]
    pub bag_clear_success_2: Handle<AudioSource>,
    #[asset("audio/bag_clear_success_3.ogg")]
    pub bag_clear_success_3: Handle<AudioSource>,
}
