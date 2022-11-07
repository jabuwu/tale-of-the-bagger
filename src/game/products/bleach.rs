use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductBleachSystem {
    Spawn,
    Spawned,
}

pub struct ProductBleachPlugin;

impl Plugin for ProductBleachPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_bleach_spawn.label(ProductBleachSystem::Spawn))
            .add_system(product_bleach_spawned.label(ProductBleachSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductBleach;

#[derive(Component)]
pub struct ProductBleachRig;

fn product_bleach_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Bleach {
            commands
                .entity(event.entity)
                .insert_bundle(SpineBundle {
                    skeleton: asset_library.spines.product_bleach.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductBleach);
        }
    }
}

fn product_bleach_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    bleach_query: Query<Entity, With<ProductBleach>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(bleach_entity) = bleach_query.get(event.entity).ok() {
            commands.entity(bleach_entity).insert(ProductBleachRig);
        }
    }
}
