use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductTorchSystem {
    Spawn,
    Spawned,
}

pub struct ProductTorchPlugin;

impl Plugin for ProductTorchPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_torch_spawn.label(ProductTorchSystem::Spawn))
            .add_system(product_torch_spawned.label(ProductTorchSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductTorch;

#[derive(Component)]
pub struct ProductTorchRig;

fn product_torch_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Torch {
            commands
                .entity(event.entity)
                .insert(SpineBundle {
                    skeleton: asset_library.spines.product_torch.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductTorch);
        }
    }
}

fn product_torch_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    torch_query: Query<Entity, With<ProductTorch>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(torch_entity) = torch_query.get(event.entity).ok() {
            commands.entity(torch_entity).insert(ProductTorchRig);
        }
    }
}
