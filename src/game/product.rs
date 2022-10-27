use bevy::prelude::*;

use crate::common::Transform2;

use super::{ConveyorItem, ConveyorSystem, ProductPlugins, DEPTH_PRODUCT};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductSystem {
    Spawn,
    Update,
}

pub struct ProductPlugin;

impl Plugin for ProductPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProductPlugins)
            .add_event::<ProductSpawnEvent>()
            .add_system(product_spawn.label(ProductSystem::Spawn))
            .add_system(
                product_update
                    .label(ProductSystem::Update)
                    .after(ConveyorSystem::ItemUpdate),
            );
    }
}

pub struct ProductSpawnEvent {
    pub entity: Entity,
    pub position: Vec2,
}

#[derive(Component)]
pub struct Product;

fn product_spawn(mut spawn_events: EventReader<ProductSpawnEvent>, mut commands: Commands) {
    for event in spawn_events.iter() {
        commands
            .entity(event.entity)
            .insert(Transform2::from_translation(event.position).with_scale(Vec2::splat(0.75)))
            .insert(DEPTH_PRODUCT)
            .insert(Product)
            .insert(ConveyorItem::default());
    }
}

fn product_update(mut product_query: Query<(&mut Transform2, &ConveyorItem), With<Product>>) {
    for (mut product_transform, product_conveyor_item) in product_query.iter_mut() {
        product_transform.translation = product_conveyor_item.position;
    }
}
