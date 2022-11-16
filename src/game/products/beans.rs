use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductBeansSystem {
    Spawn,
    Spawned,
}

pub struct ProductBeansPlugin;

impl Plugin for ProductBeansPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_beans_spawn.label(ProductBeansSystem::Spawn))
            .add_system(product_beans_spawned.label(ProductBeansSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductBeans;

#[derive(Component)]
pub struct ProductBeansRig;

fn product_beans_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::Beans {
            commands
                .entity(event.entity)
                .insert(SpineBundle {
                    skeleton: asset_library.spines.product_beans.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductBeans);
        }
    }
}

fn product_beans_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    beans_query: Query<Entity, With<ProductBeans>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(beans_entity) = beans_query.get(event.entity).ok() {
            commands.entity(beans_entity).insert(ProductBeansRig);
        }
    }
}
