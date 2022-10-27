use bevy::prelude::*;

use crate::common::{CollisionShape, Cursor, Hoverable, Transform2};

use super::{ConveyorItem, ConveyorSystem, ProductPlugins, DEPTH_PRODUCT};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductSystem {
    Spawn,
    Update,
    Drag,
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
            )
            .add_system(product_drag.label(ProductSystem::Drag));
    }
}

pub struct ProductSpawnEvent {
    pub entity: Entity,
    pub position: Vec2,
}

#[derive(Default, Component)]
pub struct Product;

#[derive(Component)]
pub struct DraggedProduct;

fn product_spawn(mut spawn_events: EventReader<ProductSpawnEvent>, mut commands: Commands) {
    for event in spawn_events.iter() {
        commands
            .entity(event.entity)
            .insert(Transform2::from_translation(event.position))
            .insert(DEPTH_PRODUCT)
            .insert(Product::default())
            .insert(ConveyorItem::default())
            .insert(Hoverable::new(
                CollisionShape::Aabb {
                    half_extents: Vec2::splat(80.),
                },
                Vec2::new(0., 50.),
            ));
    }
}

fn product_update(
    mut product_query: Query<
        (&mut Transform2, &ConveyorItem, Option<&DraggedProduct>),
        With<Product>,
    >,
    cursor: Res<Cursor>,
) {
    for (mut product_transform, product_conveyor_item, product_dragged) in product_query.iter_mut()
    {
        let dragging = product_dragged.is_some();
        let destination = if dragging {
            cursor.position + Vec2::new(0., -50.)
        } else {
            product_conveyor_item.position
        };
        product_transform.translation = destination;
    }
}

fn product_drag(
    mut commands: Commands,
    product_query: Query<(Entity, &Hoverable), With<Product>>,
    dragged_product_query: Query<Entity, With<DraggedProduct>>,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    let pressed = mouse_buttons.just_pressed(MouseButton::Left);
    let released = mouse_buttons.just_released(MouseButton::Left);
    if pressed || released {
        for dragged_product_entity in dragged_product_query.iter() {
            commands
                .entity(dragged_product_entity)
                .remove::<DraggedProduct>();
        }
        if pressed {
            for (product_entity, product_hoverable) in product_query.iter() {
                if product_hoverable.hovered {
                    commands.entity(product_entity).insert(DraggedProduct);
                    break;
                }
            }
        }
    }
}
