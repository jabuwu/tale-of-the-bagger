use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductBatteriesSystem {
    Spawn,
    Spawned,
}

pub struct ProductBatteriesPlugin;

impl Plugin for ProductBatteriesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_batteries_spawn.label(ProductBatteriesSystem::Spawn))
            .add_system(product_batteries_spawned.label(ProductBatteriesSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductBatteries;

#[derive(Component)]
pub struct ProductBatteriesRig;

fn product_batteries_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Batteries {
            commands
                .entity(event.entity)
                .insert(SpineBundle {
                    skeleton: asset_library.spines.product_batteries.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductBatteries);
        }
    }
}

fn product_batteries_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    batteries_query: Query<Entity, With<ProductBatteries>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(batteries_entity) = batteries_query.get(event.entity).ok() {
            commands
                .entity(batteries_entity)
                .insert(ProductBatteriesRig);
        }
    }
}
