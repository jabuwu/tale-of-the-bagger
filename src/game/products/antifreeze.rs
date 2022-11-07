use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductAntifreezeSystem {
    Spawn,
    Spawned,
}

pub struct ProductAntifreezePlugin;

impl Plugin for ProductAntifreezePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_antifreeze_spawn.label(ProductAntifreezeSystem::Spawn))
            .add_system(product_antifreeze_spawned.label(ProductAntifreezeSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductAntifreeze;

#[derive(Component)]
pub struct ProductAntifreezeRig;

fn product_antifreeze_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Antifreeze {
            commands
                .entity(event.entity)
                .insert_bundle(SpineBundle {
                    skeleton: asset_library.spines.product_antifreeze.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductAntifreeze);
        }
    }
}

fn product_antifreeze_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    antifreeze_query: Query<Entity, With<ProductAntifreeze>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(antifreeze_entity) = antifreeze_query.get(event.entity).ok() {
            commands
                .entity(antifreeze_entity)
                .insert(ProductAntifreezeRig);
        }
    }
}
