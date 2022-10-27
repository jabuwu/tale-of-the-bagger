use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::{Aabb, CollisionShape, GameInput, Interactable, SpineSync2, Transform2},
    AssetLibrary,
};

use super::{ConveyorItem, Product, ProductDropEvent, ProductSystem, DEPTH_BAG};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum BagSystem {
    Spawn,
    Spawned,
    Update,
    Hover,
    ProductDrop,
}

pub struct BagPlugin;

impl Plugin for BagPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BagSpawnEvent>()
            .add_system(bag_spawn.label(BagSystem::Spawn))
            .add_system(
                bag_spawned
                    .label(BagSystem::Spawned)
                    .before_spine_sync::<SpineSync2>(),
            )
            .add_system(
                bag_update
                    .label(BagSystem::Update)
                    .after(SpineSystem::Update)
                    .before(SpineSystem::Render),
            )
            .add_system(
                bag_product_drop
                    .label(BagSystem::ProductDrop)
                    .after(ProductSystem::Drag),
            );
    }
}

#[derive(Default)]
pub struct BagSpawnEvent {
    pub position: Vec2,
}

#[derive(Default, Component)]
pub struct Bag {
    slots: Vec<Entity>,
}

fn bag_spawn(
    mut spawn_events: EventReader<BagSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        commands
            .spawn_bundle(SpineBundle {
                skeleton: asset_library.spines.bag.clone(),
                ..Default::default()
            })
            .insert(Transform2::from_translation(event.position))
            .insert(DEPTH_BAG)
            .insert(SpineSync2)
            .insert(Bag::default());
    }
}

fn bag_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    mut bag_query: Query<(Entity, &mut Bag, &Spine)>,
) {
    for event in spine_ready_event.iter() {
        if let Some((bag_entity, mut bag, bag_spine)) = bag_query.get_mut(event.entity).ok() {
            if let Some(bounds) = bag_spine
                .skeleton
                .find_slot("bounds")
                .unwrap()
                .bounding_box_attachment()
            {
                let aabb = Aabb::new_from_vertices(
                    &bounds
                        .vertices2()
                        .iter()
                        .map(|vec| Vec2::from(*vec))
                        .collect::<Vec<Vec2>>(),
                )
                .unwrap();
                commands.entity(bag_entity).insert(Interactable::new(
                    CollisionShape::Aabb {
                        half_extents: aabb.half_extents,
                    },
                    aabb.translation,
                ));
            }
            for slot_name in ["slot1", "slot2", "slot3"].into_iter() {
                let slot_entity = *event.bones.get(slot_name).unwrap();
                bag.slots.push(slot_entity);
            }
        }
    }
}

fn bag_update(
    mut bag_query: Query<(&mut Spine, &Interactable), With<Bag>>,
    game_input: Res<GameInput>,
) {
    for (mut bag_spine, bag_interactable) in bag_query.iter_mut() {
        *bag_spine.skeleton.find_slot_mut("bag").unwrap().color_mut() =
            if bag_interactable.hovered(game_input.as_ref()) {
                bevy_spine::Color::new_rgba(1., 0., 0., 1.)
            } else {
                bevy_spine::Color::new_rgba(1., 1., 1., 1.)
            };
    }
}

fn bag_product_drop(
    mut drop_events: EventReader<ProductDropEvent>,
    mut product_query: Query<(Entity, &mut Transform2), With<Product>>,
    mut bag_query: Query<(&mut Bag, &mut Spine, &Interactable)>,
    mut commands: Commands,
) {
    for event in drop_events.iter() {
        if let Some((product_entity, mut product_transform)) =
            product_query.get_mut(event.entity).ok()
        {
            for (mut bag, mut bag_spine, bag_interactable) in bag_query.iter_mut() {
                if bag_interactable.contains_point(event.position) {
                    if bag.slots.len() > 0 {
                        let _ =
                            bag_spine
                                .animation_state
                                .set_animation_by_name(0, "animation", false);
                        commands.entity(product_entity).remove::<ConveyorItem>();
                        commands.entity(bag.slots[0]).add_child(product_entity);
                        bag.slots.remove(0);
                        product_transform.translation = Vec2::ZERO;
                    }
                }
            }
        }
    }
}
