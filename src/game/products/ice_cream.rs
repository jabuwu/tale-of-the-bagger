use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductIceCreamSystem {
    Spawn,
    Spawned,
}

pub struct ProductIceCreamPlugin;

impl Plugin for ProductIceCreamPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_ice_cream_spawn.label(ProductIceCreamSystem::Spawn))
            .add_system(product_ice_cream_spawned.label(ProductIceCreamSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductIceCream;

#[derive(Component)]
pub struct ProductIceCreamRig;

fn product_ice_cream_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::IceCream {
            commands
                .entity(event.entity)
                .insert(SpineBundle {
                    skeleton: asset_library.spines.product_ice_cream.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductIceCream);
        }
    }
}

fn product_ice_cream_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    ice_cream_query: Query<Entity, With<ProductIceCream>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(ice_cream_entity) = ice_cream_query.get(event.entity).ok() {
            commands.entity(ice_cream_entity).insert(ProductIceCreamRig);
        }
    }
}
