use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductAk47System {
    Spawn,
    Spawned,
}

pub struct ProductAk47Plugin;

impl Plugin for ProductAk47Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_ak47_spawn.label(ProductAk47System::Spawn))
            .add_system(product_ak47_spawned.label(ProductAk47System::Spawned));
    }
}

#[derive(Component)]
pub struct ProductAk47;

#[derive(Component)]
pub struct ProductAk47Rig;

fn product_ak47_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Ak47 {
            commands
                .entity(event.entity)
                .insert_bundle(SpineBundle {
                    skeleton: asset_library.spines.product_ak47.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductAk47);
        }
    }
}

fn product_ak47_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    ak47_query: Query<Entity, With<ProductAk47>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(ak47_entity) = ak47_query.get(event.entity).ok() {
            commands.entity(ak47_entity).insert(ProductAk47Rig);
        }
    }
}
