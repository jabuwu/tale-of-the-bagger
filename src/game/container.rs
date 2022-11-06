use bevy::prelude::*;

use crate::{
    common::{GameInput, Interactable},
    game::ProductDrag,
};

use super::{Product, ProductKind, ProductSystem};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ContainerSystem {
    Insert,
    Drag,
}

pub struct ContainerPlugin;

impl Plugin for ContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ContainerInsert>()
            .add_event::<ContainerInserted>()
            .add_system(
                container_insert
                    .label(ContainerSystem::Insert)
                    .after(ProductSystem::Drop),
            )
            .add_system(container_drag.label(ContainerSystem::Drag));
    }
}

pub struct ContainerInsert {
    pub container: Entity,
    pub product: Entity,
}

pub struct ContainerInserted {
    pub container: Entity,
    pub slot: Entity,
    pub product: Entity,
}

#[derive(Default, Component)]
pub struct Container {
    pub slots: Vec<ContainerSlot>,
    pub products: Vec<ProductKind>,
}

impl Container {
    pub fn valid_stack(&self) -> bool {
        ProductKind::valid_stack(&self.products)
    }
}

pub struct ContainerSlot {
    pub slot_entity: Entity,
    pub product_entity: Option<Entity>,
}

fn container_insert(
    mut commands: Commands,
    mut insert_events: EventReader<ContainerInsert>,
    mut inserted_events: EventWriter<ContainerInserted>,
    mut container_query: Query<(Entity, &mut Container)>,
    product_query: Query<&Product>,
) {
    for event in insert_events.iter() {
        let mut inserted = false;
        if let Some((_, mut container)) = container_query.get_mut(event.container).ok() {
            for slot in container.slots.iter_mut() {
                if slot
                    .product_entity
                    .map(|product| product == event.product)
                    .unwrap_or(false)
                {
                    break;
                }
                if slot.product_entity.is_none() {
                    slot.product_entity = Some(event.product);
                    commands.entity(slot.slot_entity).add_child(event.product);
                    inserted_events.send(ContainerInserted {
                        container: event.container,
                        slot: slot.slot_entity,
                        product: event.product,
                    });
                    inserted = true;
                    break;
                }
            }
            if inserted {
                let mut products = vec![];
                for slot in container.slots.iter() {
                    if let Some(product) = slot
                        .product_entity
                        .and_then(|entity| product_query.get(entity).ok())
                    {
                        products.push(product.kind());
                    } else {
                        break;
                    }
                }
                container.products = products;
            }
        }
        if inserted {
            for (container_entity, mut container) in container_query.iter_mut() {
                let mut updated = false;
                if container_entity != event.container {
                    for mut slot in container.slots.iter_mut() {
                        if slot
                            .product_entity
                            .map(|entity| entity == event.product)
                            .unwrap_or(false)
                        {
                            slot.product_entity = None;
                            updated = true;
                        }
                    }
                }
                if updated {
                    let mut products = vec![];
                    for slot in container.slots.iter() {
                        if let Some(product) = slot
                            .product_entity
                            .and_then(|entity| product_query.get(entity).ok())
                        {
                            products.push(product.kind());
                        } else {
                            break;
                        }
                    }
                    container.products = products;
                }
            }
        }
    }
}

fn container_drag(
    mut commands: Commands,
    container_query: Query<(&Container, &Interactable)>,
    game_input: Res<GameInput>,
) {
    for (container, container_interactable) in container_query.iter() {
        if let Some(drag_id) = container_interactable.drag_started(game_input.as_ref()) {
            for slot in container.slots.iter().rev() {
                if let Some(product_entity) = slot.product_entity {
                    commands.entity(product_entity).insert(ProductDrag(drag_id));
                    break;
                }
            }
        }
    }
}
