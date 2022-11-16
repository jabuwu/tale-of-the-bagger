use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductSkullSystem {
    Spawn,
    Spawned,
}

pub struct ProductSkullPlugin;

impl Plugin for ProductSkullPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_skull_spawn.label(ProductSkullSystem::Spawn))
            .add_system(product_skull_spawned.label(ProductSkullSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductSkull;

#[derive(Component)]
pub struct ProductSkullRig;

fn product_skull_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Skull {
            commands
                .entity(event.entity)
                .insert(SpineBundle {
                    skeleton: asset_library.spines.product_skull.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductSkull);
        }
    }
}

fn product_skull_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    skull_query: Query<Entity, With<ProductSkull>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(skull_entity) = skull_query.get(event.entity).ok() {
            commands.entity(skull_entity).insert(ProductSkullRig);
        }
    }
}
