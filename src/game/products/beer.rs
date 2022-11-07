use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductBeerSystem {
    Spawn,
    Spawned,
}

pub struct ProductBeerPlugin;

impl Plugin for ProductBeerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_beer_spawn.label(ProductBeerSystem::Spawn))
            .add_system(product_beer_spawned.label(ProductBeerSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductBeer;

#[derive(Component)]
pub struct ProductBeerRig;

fn product_beer_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Beer {
            commands
                .entity(event.entity)
                .insert_bundle(SpineBundle {
                    skeleton: asset_library.spines.product_beer.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductBeer);
        }
    }
}

fn product_beer_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    beer_query: Query<Entity, With<ProductBeer>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(beer_entity) = beer_query.get(event.entity).ok() {
            commands.entity(beer_entity).insert(ProductBeerRig);
        }
    }
}
