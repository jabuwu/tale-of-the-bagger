use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductBoilingWaterSystem {
    Spawn,
    Spawned,
}

pub struct ProductBoilingWaterPlugin;

impl Plugin for ProductBoilingWaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_boiling_water_spawn.label(ProductBoilingWaterSystem::Spawn))
            .add_system(product_boiling_water_spawned.label(ProductBoilingWaterSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductBoilingWater;

#[derive(Component)]
pub struct ProductBoilingWaterRig;

fn product_boiling_water_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::BoilingWater {
            commands
                .entity(event.entity)
                .insert_bundle(SpineBundle {
                    skeleton: asset_library.spines.product_boiling_water.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductBoilingWater);
        }
    }
}

fn product_boiling_water_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    boiling_water_query: Query<Entity, With<ProductBoilingWater>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(boiling_water_entity) = boiling_water_query.get(event.entity).ok() {
            commands
                .entity(boiling_water_entity)
                .insert(ProductBoilingWaterRig);
        }
    }
}
