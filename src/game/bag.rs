use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_spine::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

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
    Clear,
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
            )
            .add_system(
                bag_clear
                    .label(BagSystem::Clear)
                    .after(BagSystem::ProductDrop),
            );
    }
}

#[derive(Default)]
pub struct BagSpawnEvent {
    pub position: Vec2,
}

#[derive(Default, Component)]
pub struct Bag {
    slots: Vec<BagSlot>,
}

pub struct BagSlot {
    slot_entity: Entity,
    product_entity: Option<Entity>,
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
                bag.slots.push(BagSlot {
                    slot_entity,
                    product_entity: None,
                });
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
                bevy_spine::Color::new_rgba(1.3, 1.3, 1.3, 1.)
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
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
) {
    for event in drop_events.iter() {
        if let Some((product_entity, mut product_transform)) =
            product_query.get_mut(event.entity).ok()
        {
            for (mut bag, mut bag_spine, bag_interactable) in bag_query.iter_mut() {
                if bag_interactable.contains_point(event.position) {
                    for slot in bag.slots.iter_mut() {
                        if slot.product_entity.is_none() {
                            let _ = bag_spine.animation_state.set_animation_by_name(
                                0,
                                "animation",
                                false,
                            );
                            commands.entity(product_entity).remove::<ConveyorItem>();
                            commands.entity(slot.slot_entity).add_child(product_entity);
                            product_transform.translation = Vec2::ZERO;
                            slot.product_entity = Some(product_entity);
                            audio.play(
                                [
                                    asset_library.audio.bag_insert_1.clone(),
                                    asset_library.audio.bag_insert_2.clone(),
                                    asset_library.audio.bag_insert_3.clone(),
                                    asset_library.audio.bag_insert_4.clone(),
                                    asset_library.audio.bag_insert_5.clone(),
                                    asset_library.audio.bag_insert_6.clone(),
                                    asset_library.audio.bag_insert_7.clone(),
                                    asset_library.audio.bag_insert_8.clone(),
                                    asset_library.audio.bag_insert_9.clone(),
                                    asset_library.audio.bag_insert_10.clone(),
                                    asset_library.audio.bag_insert_11.clone(),
                                ]
                                .choose(&mut thread_rng())
                                .unwrap()
                                .clone(),
                            );
                            break;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Default)]
struct BagClearLocal {
    audio_track: usize,
}

fn bag_clear(
    mut bag_query: Query<&mut Bag>,
    mut commands: Commands,
    mut local: Local<BagClearLocal>,
    asset_library: Res<AssetLibrary>,
    audio: Res<Audio>,
) {
    for mut bag in bag_query.iter_mut() {
        if bag.slots.len() > 0
            && bag
                .slots
                .get(bag.slots.len() - 1)
                .unwrap()
                .product_entity
                .is_some()
        {
            for slot in bag.slots.iter_mut() {
                if let Some(product_entity) = slot.product_entity {
                    commands.entity(product_entity).despawn_recursive();
                }
                slot.product_entity = None;
            }
            audio.play(
                [
                    asset_library.audio.bag_clear_success_1.clone(),
                    asset_library.audio.bag_clear_success_2.clone(),
                    asset_library.audio.bag_clear_success_3.clone(),
                ]
                .into_iter()
                .nth(local.audio_track)
                .unwrap()
                .clone(),
            );
            local.audio_track = (local.audio_track + 1) % 3;
        }
    }
}
