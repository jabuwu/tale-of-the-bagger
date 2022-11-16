use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductSoupSystem {
    Spawn,
    Spawned,
}

pub struct ProductSoupPlugin;

impl Plugin for ProductSoupPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_soup_spawn.label(ProductSoupSystem::Spawn))
            .add_system(product_soup_spawned.label(ProductSoupSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductSoup;

#[derive(Component)]
pub struct ProductSoupRig;

fn product_soup_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Soup {
            commands
                .entity(event.entity)
                .insert(SpineBundle {
                    skeleton: asset_library.spines.product_soup.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductSoup);
        }
    }
}

fn product_soup_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    soup_query: Query<Entity, With<ProductSoup>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(soup_entity) = soup_query.get(event.entity).ok() {
            commands.entity(soup_entity).insert(ProductSoupRig);
        }
    }
}
