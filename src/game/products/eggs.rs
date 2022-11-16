use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductEggsSystem {
    Spawn,
    Spawned,
}

pub struct ProductEggsPlugin;

impl Plugin for ProductEggsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_eggs_spawn.label(ProductEggsSystem::Spawn))
            .add_system(product_eggs_spawned.label(ProductEggsSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductEggs;

#[derive(Component)]
pub struct ProductEggsRig;

fn product_eggs_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Eggs {
            commands
                .entity(event.entity)
                .insert(SpineBundle {
                    skeleton: asset_library.spines.product_eggs.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductEggs);
        }
    }
}

fn product_eggs_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    eggs_query: Query<Entity, With<ProductEggs>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(eggs_entity) = eggs_query.get(event.entity).ok() {
            commands.entity(eggs_entity).insert(ProductEggsRig);
        }
    }
}
