use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductWatermelonSystem {
    Spawn,
    Spawned,
}

pub struct ProductWatermelonPlugin;

impl Plugin for ProductWatermelonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_watermelon_spawn.label(ProductWatermelonSystem::Spawn))
            .add_system(product_watermelon_spawned.label(ProductWatermelonSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductWatermelon;

#[derive(Component)]
pub struct ProductWatermelonRig;

fn product_watermelon_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Watermelon {
            commands
                .entity(event.entity)
                .insert(SpineBundle {
                    skeleton: asset_library.spines.product_watermelon.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductWatermelon);
        }
    }
}

fn product_watermelon_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    watermelon_query: Query<Entity, With<ProductWatermelon>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(watermelon_entity) = watermelon_query.get(event.entity).ok() {
            commands
                .entity(watermelon_entity)
                .insert(ProductWatermelonRig);
        }
    }
}
