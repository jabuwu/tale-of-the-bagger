use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductTacoSystem {
    Spawn,
    Spawned,
}

pub struct ProductTacoPlugin;

impl Plugin for ProductTacoPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_taco_spawn.label(ProductTacoSystem::Spawn))
            .add_system(product_taco_spawned.label(ProductTacoSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductTaco;

#[derive(Component)]
pub struct ProductTacoRig;

fn product_taco_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Taco {
            commands
                .entity(event.entity)
                .insert(SpineBundle {
                    skeleton: asset_library.spines.product_taco.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductTaco);
        }
    }
}

fn product_taco_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    taco_query: Query<Entity, With<ProductTaco>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(taco_entity) = taco_query.get(event.entity).ok() {
            commands.entity(taco_entity).insert(ProductTacoRig);
        }
    }
}
