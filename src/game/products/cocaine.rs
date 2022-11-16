use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductCocaineSystem {
    Spawn,
    Spawned,
}

pub struct ProductCocainePlugin;

impl Plugin for ProductCocainePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_cocaine_spawn.label(ProductCocaineSystem::Spawn))
            .add_system(product_cocaine_spawned.label(ProductCocaineSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductCocaine;

#[derive(Component)]
pub struct ProductCocaineRig;

fn product_cocaine_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Cocaine {
            commands
                .entity(event.entity)
                .insert(SpineBundle {
                    skeleton: asset_library.spines.product_cocaine.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductCocaine);
        }
    }
}

fn product_cocaine_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    cocaine_query: Query<Entity, With<ProductCocaine>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(cocaine_entity) = cocaine_query.get(event.entity).ok() {
            commands.entity(cocaine_entity).insert(ProductCocaineRig);
        }
    }
}
