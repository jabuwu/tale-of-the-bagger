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

    #[asset("textures/icons/cold.png")]
    pub icon_cold: Handle<Image>,
    #[asset("textures/icons/hot.png")]
    pub icon_hot: Handle<Image>,
    #[asset("textures/icons/fresh.png")]
    pub icon_fresh: Handle<Image>,
    #[asset("textures/icons/meat.png")]
    pub icon_meat: Handle<Image>,
    #[asset("textures/icons/fragile.png")]
    pub icon_fragile: Handle<Image>,
    #[asset("textures/icons/heavy.png")]
    pub icon_heavy: Handle<Image>,
    #[asset("textures/icons/toxic.png")]
    pub icon_toxic: Handle<Image>,
    #[asset("textures/icons/illicit.png")]
    pub icon_illicit: Handle<Image>,
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
    #[spine_asset("spines/health")]
    pub health: Handle<SkeletonData>,

    #[spine_asset("spines/products/ak47")]
    pub product_ak47: Handle<SkeletonData>,
    #[spine_asset("spines/products/antifreeze")]
    pub product_antifreeze: Handle<SkeletonData>,
    #[spine_asset("spines/products/avocado")]
    pub product_avocado: Handle<SkeletonData>,
    #[spine_asset("spines/products/bacon")]
    pub product_bacon: Handle<SkeletonData>,
    #[spine_asset("spines/products/batteries")]
    pub product_batteries: Handle<SkeletonData>,
    #[spine_asset("spines/products/beans")]
    pub product_beans: Handle<SkeletonData>,
    #[spine_asset("spines/products/beer")]
    pub product_beer: Handle<SkeletonData>,
    #[spine_asset("spines/products/bleach")]
    pub product_bleach: Handle<SkeletonData>,
    #[spine_asset("spines/products/boiling-water")]
    pub product_boiling_water: Handle<SkeletonData>,
    #[spine_asset("spines/products/bowling-ball")]
    pub product_bowling_ball: Handle<SkeletonData>,
    #[spine_asset("spines/products/bread")]
    pub product_bread: Handle<SkeletonData>,
    #[spine_asset("spines/products/chicken")]
    pub product_chicken: Handle<SkeletonData>,
    #[spine_asset("spines/products/cinder")]
    pub product_cinder: Handle<SkeletonData>,
    #[spine_asset("spines/products/cocaine")]
    pub product_cocaine: Handle<SkeletonData>,
    #[spine_asset("spines/products/coffee")]
    pub product_coffee: Handle<SkeletonData>,
    #[spine_asset("spines/products/eggs")]
    pub product_eggs: Handle<SkeletonData>,
    #[spine_asset("spines/products/good-stuff")]
    pub product_good_stuff: Handle<SkeletonData>,
    #[spine_asset("spines/products/heart")]
    pub product_heart: Handle<SkeletonData>,
    #[spine_asset("spines/products/ice")]
    pub product_ice: Handle<SkeletonData>,
    #[spine_asset("spines/products/ice-cream")]
    pub product_ice_cream: Handle<SkeletonData>,
    #[spine_asset("spines/products/jerky")]
    pub product_jerky: Handle<SkeletonData>,
    #[spine_asset("spines/products/katana")]
    pub product_katana: Handle<SkeletonData>,
    #[spine_asset("spines/products/ketchup")]
    pub product_ketchup: Handle<SkeletonData>,
    #[spine_asset("spines/products/milk")]
    pub product_milk: Handle<SkeletonData>,
    #[spine_asset("spines/products/plate")]
    pub product_plate: Handle<SkeletonData>,
    #[spine_asset("spines/products/rocket-fuel")]
    pub product_rocket_fuel: Handle<SkeletonData>,
    #[spine_asset("spines/products/skull")]
    pub product_skull: Handle<SkeletonData>,
    #[spine_asset("spines/products/soup")]
    pub product_soup: Handle<SkeletonData>,
    #[spine_asset("spines/products/taco")]
    pub product_taco: Handle<SkeletonData>,
    #[spine_asset("spines/products/torch")]
    pub product_torch: Handle<SkeletonData>,
    #[spine_asset("spines/products/watermelon")]
    pub product_watermelon: Handle<SkeletonData>,
    #[spine_asset("spines/products/xxx")]
    pub product_xxx: Handle<SkeletonData>,
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

    #[asset("audio/bag_clear_error.ogg")]
    pub bag_clear_error: Handle<AudioSource>,
}
