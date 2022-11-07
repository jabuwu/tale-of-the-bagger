use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductAvocadoSystem {
    Spawn,
    Spawned,
}

pub struct ProductAvocadoPlugin;

impl Plugin for ProductAvocadoPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_avocado_spawn.label(ProductAvocadoSystem::Spawn))
            .add_system(product_avocado_spawned.label(ProductAvocadoSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductAvocado;

#[derive(Component)]
pub struct ProductAvocadoRig;

fn product_avocado_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Avocado {
            commands
                .entity(event.entity)
                .insert_bundle(SpineBundle {
                    skeleton: asset_library.spines.product_avocado.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductAvocado);
        }
    }
}

fn product_avocado_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    avocado_query: Query<Entity, With<ProductAvocado>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(avocado_entity) = avocado_query.get(event.entity).ok() {
            commands.entity(avocado_entity).insert(ProductAvocadoRig);
        }
    }
}
