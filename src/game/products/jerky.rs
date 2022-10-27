use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{common::SpineSync2, game::ProductSpawnEvent, AssetLibrary};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductJerkySystem {
    Spawn,
    Update,
}

pub struct ProductJerkyPlugin;

impl Plugin for ProductJerkyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_jerky_spawn.label(ProductJerkySystem::Spawn));
    }
}

#[derive(Component)]
pub struct ProductJerky;

fn product_jerky_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        commands
            .entity(event.entity)
            .insert_bundle(SpineBundle {
                skeleton: asset_library.spines.product_jerky.clone(),
                ..Default::default()
            })
            .insert(SpineSync2);
    }
}
