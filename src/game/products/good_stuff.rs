use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductGoodStuffSystem {
    Spawn,
    Spawned,
}

pub struct ProductGoodStuffPlugin;

impl Plugin for ProductGoodStuffPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_good_stuff_spawn.label(ProductGoodStuffSystem::Spawn))
            .add_system(product_good_stuff_spawned.label(ProductGoodStuffSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductGoodStuff;

#[derive(Component)]
pub struct ProductGoodStuffRig;

fn product_good_stuff_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::GoodStuff {
            commands
                .entity(event.entity)
                .insert(SpineBundle {
                    skeleton: asset_library.spines.product_good_stuff.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductGoodStuff);
        }
    }
}

fn product_good_stuff_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    good_stuff_query: Query<Entity, With<ProductGoodStuff>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(good_stuff_entity) = good_stuff_query.get(event.entity).ok() {
            commands
                .entity(good_stuff_entity)
                .insert(ProductGoodStuffRig);
        }
    }
}
