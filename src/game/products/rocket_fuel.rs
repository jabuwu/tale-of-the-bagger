use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductRocketFuelSystem {
    Spawn,
    Spawned,
}

pub struct ProductRocketFuelPlugin;

impl Plugin for ProductRocketFuelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_rocket_fuel_spawn.label(ProductRocketFuelSystem::Spawn))
            .add_system(product_rocket_fuel_spawned.label(ProductRocketFuelSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductRocketFuel;

#[derive(Component)]
pub struct ProductRocketFuelRig;

fn product_rocket_fuel_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::RocketFuel {
            commands
                .entity(event.entity)
                .insert_bundle(SpineBundle {
                    skeleton: asset_library.spines.product_rocket_fuel.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductRocketFuel);
        }
    }
}

fn product_rocket_fuel_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    rocket_fuel_query: Query<Entity, With<ProductRocketFuel>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(rocket_fuel_entity) = rocket_fuel_query.get(event.entity).ok() {
            commands
                .entity(rocket_fuel_entity)
                .insert(ProductRocketFuelRig);
        }
    }
}
