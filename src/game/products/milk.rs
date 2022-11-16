use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductMilkSystem {
    Spawn,
    Spawned,
}

pub struct ProductMilkPlugin;

impl Plugin for ProductMilkPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_milk_spawn.label(ProductMilkSystem::Spawn))
            .add_system(product_milk_spawned.label(ProductMilkSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductMilk;

#[derive(Component)]
pub struct ProductMilkRig;

fn product_milk_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Milk {
            commands
                .entity(event.entity)
                .insert(SpineBundle {
                    skeleton: asset_library.spines.product_milk.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductMilk);
        }
    }
}

fn product_milk_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    milk_query: Query<Entity, With<ProductMilk>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(milk_entity) = milk_query.get(event.entity).ok() {
            commands.entity(milk_entity).insert(ProductMilkRig);
        }
    }
}
