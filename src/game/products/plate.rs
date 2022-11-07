use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductPlateSystem {
    Spawn,
    Spawned,
}

pub struct ProductPlatePlugin;

impl Plugin for ProductPlatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_plate_spawn.label(ProductPlateSystem::Spawn))
            .add_system(product_plate_spawned.label(ProductPlateSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductPlate;

#[derive(Component)]
pub struct ProductPlateRig;

fn product_plate_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Plate {
            commands
                .entity(event.entity)
                .insert_bundle(SpineBundle {
                    skeleton: asset_library.spines.product_plate.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductPlate);
        }
    }
}

fn product_plate_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    plate_query: Query<Entity, With<ProductPlate>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(plate_entity) = plate_query.get(event.entity).ok() {
            commands.entity(plate_entity).insert(ProductPlateRig);
        }
    }
}
