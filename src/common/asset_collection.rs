use bevy::prelude::*;
use bevy_spine::prelude::*;

pub use tale_of_the_bagger_macros::AssetCollection;

pub trait AssetCollection {
    fn load_assets(&mut self, skeletons: &mut Assets<SkeletonData>, asset_server: &AssetServer);
}
