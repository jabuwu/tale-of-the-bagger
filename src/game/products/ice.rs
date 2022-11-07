use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductIceSystem {
    Spawn,
    Spawned,
}

pub struct ProductIcePlugin;

impl Plugin for ProductIcePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_ice_spawn.label(ProductIceSystem::Spawn))
            .add_system(product_ice_spawned.label(ProductIceSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductIce;

#[derive(Component)]
pub struct ProductIceRig;

fn product_ice_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Ice {
            commands
                .entity(event.entity)
                .insert_bundle(SpineBundle {
                    skeleton: asset_library.spines.product_ice.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductIce);
        }
    }
}

fn product_ice_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    ice_query: Query<Entity, With<ProductIce>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(ice_entity) = ice_query.get(event.entity).ok() {
            commands.entity(ice_entity).insert(ProductIceRig);
        }
    }
}
