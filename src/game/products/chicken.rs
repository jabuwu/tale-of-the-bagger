use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductChickenSystem {
    Spawn,
    Spawned,
}

pub struct ProductChickenPlugin;

impl Plugin for ProductChickenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_chicken_spawn.label(ProductChickenSystem::Spawn))
            .add_system(product_chicken_spawned.label(ProductChickenSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductChicken;

#[derive(Component)]
pub struct ProductChickenRig;

fn product_chicken_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Chicken {
            commands
                .entity(event.entity)
                .insert(SpineBundle {
                    skeleton: asset_library.spines.product_chicken.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductChicken);
        }
    }
}

fn product_chicken_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    chicken_query: Query<Entity, With<ProductChicken>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(chicken_entity) = chicken_query.get(event.entity).ok() {
            commands.entity(chicken_entity).insert(ProductChickenRig);
        }
    }
}
