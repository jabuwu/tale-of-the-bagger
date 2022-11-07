use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductCinderSystem {
    Spawn,
    Spawned,
}

pub struct ProductCinderPlugin;

impl Plugin for ProductCinderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_cinder_spawn.label(ProductCinderSystem::Spawn))
            .add_system(product_cinder_spawned.label(ProductCinderSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductCinder;

#[derive(Component)]
pub struct ProductCinderRig;

fn product_cinder_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Cinder {
            commands
                .entity(event.entity)
                .insert_bundle(SpineBundle {
                    skeleton: asset_library.spines.product_cinder.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductCinder);
        }
    }
}

fn product_cinder_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    cinder_query: Query<Entity, With<ProductCinder>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(cinder_entity) = cinder_query.get(event.entity).ok() {
            commands.entity(cinder_entity).insert(ProductCinderRig);
        }
    }
}
