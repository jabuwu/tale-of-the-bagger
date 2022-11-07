use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductHeartSystem {
    Spawn,
    Spawned,
}

pub struct ProductHeartPlugin;

impl Plugin for ProductHeartPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_heart_spawn.label(ProductHeartSystem::Spawn))
            .add_system(product_heart_spawned.label(ProductHeartSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductHeart;

#[derive(Component)]
pub struct ProductHeartRig;

fn product_heart_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Heart {
            commands
                .entity(event.entity)
                .insert_bundle(SpineBundle {
                    skeleton: asset_library.spines.product_heart.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductHeart);
        }
    }
}

fn product_heart_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    heart_query: Query<Entity, With<ProductHeart>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(heart_entity) = heart_query.get(event.entity).ok() {
            commands.entity(heart_entity).insert(ProductHeartRig);
        }
    }
}
