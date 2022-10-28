use bevy::prelude::*;

use crate::common::{CollisionShape, DepthLayer, GameInput, Interactable, Transform2};

use super::{
    BagSystem, ConveyorItem, ConveyorSystem, ProductPlugins, DEPTH_PRODUCT, DEPTH_PRODUCT_DRAGGING,
};

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
            .add_event::<ProductDropEvent>()
            .add_system(product_spawn.label(ProductSystem::Spawn))
            .add_system(
                product_update
                    .label(ProductSystem::Update)
                    .after(ConveyorSystem::ItemUpdate)
                    .before(BagSystem::ProductDrop),
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
pub struct ProductDrag(u64);

#[derive(Debug)]
pub struct ProductDropEvent {
    pub entity: Entity,
    pub position: Vec2,
}

fn product_spawn(mut spawn_events: EventReader<ProductSpawnEvent>, mut commands: Commands) {
    for event in spawn_events.iter() {
        commands
            .entity(event.entity)
            .insert(Transform2::from_translation(event.position))
            .insert(DEPTH_PRODUCT)
            .insert(Product::default())
            .insert(ConveyorItem::default())
            .insert(Interactable::new(
                CollisionShape::Aabb {
                    half_extents: Vec2::splat(80.),
                },
                Vec2::ZERO,
            ));
    }
}

fn product_update(
    mut product_query: Query<
        (
            &mut Transform2,
            &mut DepthLayer,
            Option<&ConveyorItem>,
            Option<&ProductDrag>,
        ),
        With<Product>,
    >,
    game_input: Res<GameInput>,
    time: Res<Time>,
) {
    for (mut product_transform, mut product_depth_layer, product_conveyor_item, product_drag) in
        product_query.iter_mut()
    {
        let destination = if let Some(drag_position) =
            product_drag.and_then(|drag| game_input.drag_position(drag.0))
        {
            drag_position
        } else if let Some(product_conveyor_item) = product_conveyor_item {
            product_conveyor_item.position
        } else {
            Vec2::ZERO
        };
        product_transform.translation = product_transform
            .translation
            .lerp(destination, time.delta_seconds() * 25.);
        *product_depth_layer = if product_drag.is_some() {
            DEPTH_PRODUCT_DRAGGING
        } else {
            DEPTH_PRODUCT
        };
    }
}

fn product_drag(
    mut commands: Commands,
    mut drop_events: EventWriter<ProductDropEvent>,
    product_query: Query<
        (Entity, &Interactable),
        (With<Product>, With<ConveyorItem>, Without<ProductDrag>),
    >,
    product_drag_query: Query<(Entity, &ProductDrag)>,
    game_input: Res<GameInput>,
) {
    for (product_entity, product_interactable) in product_query.iter() {
        if let Some(drag) = product_interactable.drag_started(game_input.as_ref()) {
            commands.entity(product_entity).insert(ProductDrag(drag));
            break;
        }
    }
    for (product_drag_entity, product_drag) in product_drag_query.iter() {
        if game_input.drag_ended(product_drag.0) {
            commands.entity(product_drag_entity).remove::<ProductDrag>();
            drop_events.send(ProductDropEvent {
                entity: product_drag_entity,
                position: game_input.drag_position(product_drag.0).unwrap(),
            });
        }
    }
}
