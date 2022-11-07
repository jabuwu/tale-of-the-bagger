use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductKatanaSystem {
    Spawn,
    Spawned,
}

pub struct ProductKatanaPlugin;

impl Plugin for ProductKatanaPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_katana_spawn.label(ProductKatanaSystem::Spawn))
            .add_system(product_katana_spawned.label(ProductKatanaSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductKatana;

#[derive(Component)]
pub struct ProductKatanaRig;

fn product_katana_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Katana {
            commands
                .entity(event.entity)
                .insert_bundle(SpineBundle {
                    skeleton: asset_library.spines.product_katana.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductKatana);
        }
    }
}

fn product_katana_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    katana_query: Query<Entity, With<ProductKatana>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(katana_entity) = katana_query.get(event.entity).ok() {
            commands.entity(katana_entity).insert(ProductKatanaRig);
        }
    }
}
