use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductBreadSystem {
    Spawn,
    Spawned,
}

pub struct ProductBreadPlugin;

impl Plugin for ProductBreadPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_bread_spawn.label(ProductBreadSystem::Spawn))
            .add_system(product_bread_spawned.label(ProductBreadSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductBread;

#[derive(Component)]
pub struct ProductBreadRig;

fn product_bread_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Bread {
            commands
                .entity(event.entity)
                .insert(SpineBundle {
                    skeleton: asset_library.spines.product_bread.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductBread);
        }
    }
}

fn product_bread_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    bread_query: Query<Entity, With<ProductBread>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(bread_entity) = bread_query.get(event.entity).ok() {
            commands.entity(bread_entity).insert(ProductBreadRig);
        }
    }
}
