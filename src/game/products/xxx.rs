use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductXxxSystem {
    Spawn,
    Spawned,
}

pub struct ProductXxxPlugin;

impl Plugin for ProductXxxPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_xxx_spawn.label(ProductXxxSystem::Spawn))
            .add_system(product_xxx_spawned.label(ProductXxxSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductXxx;

#[derive(Component)]
pub struct ProductXxxRig;

fn product_xxx_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Xxx {
            commands
                .entity(event.entity)
                .insert(SpineBundle {
                    skeleton: asset_library.spines.product_xxx.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductXxx);
        }
    }
}

fn product_xxx_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    xxx_query: Query<Entity, With<ProductXxx>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(xxx_entity) = xxx_query.get(event.entity).ok() {
            commands.entity(xxx_entity).insert(ProductXxxRig);
        }
    }
}
