use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductBaconSystem {
    Spawn,
    Spawned,
}

pub struct ProductBaconPlugin;

impl Plugin for ProductBaconPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_bacon_spawn.label(ProductBaconSystem::Spawn))
            .add_system(product_bacon_spawned.label(ProductBaconSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductBacon;

#[derive(Component)]
pub struct ProductBaconRig;

fn product_bacon_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Bacon {
            commands
                .entity(event.entity)
                .insert_bundle(SpineBundle {
                    skeleton: asset_library.spines.product_bacon.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductBacon);
        }
    }
}

fn product_bacon_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    bacon_query: Query<Entity, With<ProductBacon>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(bacon_entity) = bacon_query.get(event.entity).ok() {
            commands.entity(bacon_entity).insert(ProductBaconRig);
        }
    }
}
