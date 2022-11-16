use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductCoffeeSystem {
    Spawn,
    Spawned,
}

pub struct ProductCoffeePlugin;

impl Plugin for ProductCoffeePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_coffee_spawn.label(ProductCoffeeSystem::Spawn))
            .add_system(product_coffee_spawned.label(ProductCoffeeSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductCoffee;

#[derive(Component)]
pub struct ProductCoffeeRig;

fn product_coffee_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Coffee {
            commands
                .entity(event.entity)
                .insert(SpineBundle {
                    skeleton: asset_library.spines.product_coffee.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductCoffee);
        }
    }
}

fn product_coffee_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    coffee_query: Query<Entity, With<ProductCoffee>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(coffee_entity) = coffee_query.get(event.entity).ok() {
            commands.entity(coffee_entity).insert(ProductCoffeeRig);
        }
    }
}
