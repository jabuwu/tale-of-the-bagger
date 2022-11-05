use bevy::prelude::*;

use crate::{
    common::{
        CollisionShape, DepthLayer, GameInput, Interactable, SecondOrderController, Transform2,
    },
    AssetLibrary,
};

use super::{
    Container, ContainerInsert, ContainerInserted, ContainerSystem, ConveyorItem, ConveyorSystem,
    ProductPlugins, DEPTH_PRODUCT, DEPTH_PRODUCT_DRAGGING, DEPTH_PRODUCT_ICON,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductSystem {
    Spawn,
    Update,
    Drag,
    Drop,
    Inserted,
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
                    .after(ConveyorSystem::ItemUpdate)
                    .before(ProductSystem::Inserted),
            )
            .add_system(product_drag.label(ProductSystem::Drag))
            .add_system(
                product_drop
                    .label(ProductSystem::Drop)
                    .before(ProductSystem::Drag),
            )
            .add_system(
                product_inserted
                    .label(ProductSystem::Inserted)
                    .after(ContainerSystem::Insert),
            );
    }
}

pub struct ProductSpawnEvent {
    pub entity: Entity,
    pub position: Vec2,
}

#[derive(Component)]
pub struct Product {
    scale_controller: SecondOrderController<f32>,
}

impl Default for Product {
    fn default() -> Self {
        Self {
            scale_controller: SecondOrderController::new(1., 4., 0.5, -3.2),
        }
    }
}

#[derive(Component)]
pub struct ProductDrag(pub u64);

fn product_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
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
            ))
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        texture: asset_library.textures.icon_meat.clone(),
                        ..Default::default()
                    })
                    .insert(Transform2::from_xy(60., -50.).with_scale(Vec2::splat(0.75)))
                    .insert(DEPTH_PRODUCT_ICON);
            });
    }
}

fn product_update(
    mut product_query: Query<(
        &mut Product,
        &mut Transform2,
        &mut DepthLayer,
        Option<&ConveyorItem>,
        Option<&ProductDrag>,
        Option<&Parent>,
    )>,
    global_transform_query: Query<&GlobalTransform>,
    game_input: Res<GameInput>,
    time: Res<Time>,
) {
    for (
        mut product,
        mut product_transform,
        mut product_depth_layer,
        product_conveyor_item,
        product_drag,
        product_parent,
    ) in product_query.iter_mut()
    {
        let parent_translation = if let Some(product_parent) = product_parent {
            if let Some(product_parent_transform) =
                global_transform_query.get(product_parent.get()).ok()
            {
                product_parent_transform.translation().truncate()
            } else {
                Vec2::ZERO
            }
        } else {
            Vec2::ZERO
        };
        let destination = if let Some(drag_position) = product_drag.and_then(|drag| {
            game_input
                .drag_position(drag.0)
                .map(|position| position - parent_translation)
        }) {
            drag_position
        } else if let Some(product_conveyor_item) = product_conveyor_item {
            product_conveyor_item.position
        } else {
            Vec2::ZERO
        };
        product_transform.translation = product_transform
            .translation
            .lerp(destination, time.delta_seconds() * 25.);
        product_transform.scale = Vec2::splat(product.scale_controller.update(
            if product_drag.is_some() { 1.4 } else { 1. },
            time.delta_seconds(),
        ));
        *product_depth_layer = if product_drag.is_some() {
            DEPTH_PRODUCT_DRAGGING
        } else {
            DEPTH_PRODUCT
        };
    }
}

fn product_drag(
    mut commands: Commands,
    product_query: Query<
        (Entity, &Interactable),
        (With<Product>, With<ConveyorItem>, Without<ProductDrag>),
    >,
    game_input: Res<GameInput>,
) {
    for (product_entity, product_interactable) in product_query.iter() {
        if let Some(drag) = product_interactable.drag_started(game_input.as_ref()) {
            commands.entity(product_entity).insert(ProductDrag(drag));
            break;
        }
    }
}

fn product_drop(
    mut commands: Commands,
    mut attach_events: EventWriter<ContainerInsert>,
    product_drag_query: Query<(Entity, &ProductDrag)>,
    container_query: Query<(Entity, &Interactable), With<Container>>,
    game_input: Res<GameInput>,
) {
    for (product_drag_entity, product_drag) in product_drag_query.iter() {
        if game_input.drag_ended(product_drag.0) {
            for (container_entity, container_interactable) in container_query.iter() {
                if container_interactable.dragging_within(game_input.as_ref(), product_drag.0) {
                    attach_events.send(ContainerInsert {
                        container: container_entity,
                        product: product_drag_entity,
                    });
                    break;
                }
            }
            commands.entity(product_drag_entity).remove::<ProductDrag>();
        }
    }
}

fn product_inserted(
    mut inserted_events: EventReader<ContainerInserted>,
    mut commands: Commands,
    mut transform_query: Query<&mut Transform2>,
) {
    for event in inserted_events.iter() {
        commands.entity(event.product).remove::<ConveyorItem>();
        if let Some(mut product_transform) = transform_query.get_mut(event.product).ok() {
            product_transform.translation = Vec2::ZERO;
        }
    }
}
