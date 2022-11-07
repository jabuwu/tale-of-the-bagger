use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_spine::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    common::{Aabb, CollisionShape, GameInput, Interactable, SpineSync2, Transform2},
    AssetLibrary,
};

use super::{
    Container, ContainerInserted, ContainerSlot, ContainerSystem, HealthDamageEvent, ProductSystem,
    DEPTH_BAG,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum BagSystem {
    Spawn,
    Spawned,
    Update,
    Hover,
    Inserted,
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
                    .after(ProductSystem::DropCandidates)
                    .before(SpineSystem::Render),
            )
            .add_system(
                bag_inserted
                    .label(BagSystem::Inserted)
                    .after(ContainerSystem::Insert),
            )
            .add_system(bag_clear.label(BagSystem::Clear).after(BagSystem::Inserted));
    }
}

#[derive(Default)]
pub struct BagSpawnEvent {
    pub position: Vec2,
}

#[derive(Default, Component)]
pub struct Bag;

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
    bag_query: Query<(Entity, &Spine), With<Bag>>,
) {
    for event in spine_ready_event.iter() {
        if let Some((bag_entity, bag_spine)) = bag_query.get(event.entity).ok() {
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
            let mut container = Container::default();
            for slot_name in ["slot1", "slot2", "slot3"].into_iter() {
                let slot_entity = *event.bones.get(slot_name).unwrap();
                container.slots.push(ContainerSlot {
                    slot_entity,
                    product_entity: None,
                });
            }
            commands.entity(bag_entity).insert(container);
        }
    }
}

fn bag_update(
    mut bag_query: Query<(&mut Spine, &Container, &Interactable), With<Bag>>,
    game_input: Res<GameInput>,
) {
    for (mut bag_spine, bag_container, bag_interactable) in bag_query.iter_mut() {
        let mut color = if bag_container.valid_stack_with_candidates() {
            Color::WHITE
        } else {
            Color::RED
        };
        if bag_interactable.hovered(game_input.as_ref()) {
            color *= 1.3;
        }
        color.set_a(1.);
        *bag_spine.skeleton.find_slot_mut("bag").unwrap().color_mut() =
            bevy_spine::Color::new_rgba(color.r(), color.g(), color.b(), color.a());
    }
}

fn bag_inserted(
    mut inserted_events: EventReader<ContainerInserted>,
    mut bag_query: Query<&mut Spine>,
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
) {
    for event in inserted_events.iter() {
        if let Some(mut bag_spine) = bag_query.get_mut(event.container).ok() {
            let _ = bag_spine
                .animation_state
                .set_animation_by_name(0, "animation", false);
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
        }
    }
}

#[derive(Default)]
struct BagClearLocal {
    audio_track: usize,
}

// TODO: some of this logic should probably be controlled by container and not bag
fn bag_clear(
    mut bag_query: Query<&mut Container, With<Bag>>,
    mut commands: Commands,
    mut local: Local<BagClearLocal>,
    mut health_damage_events: EventWriter<HealthDamageEvent>,
    asset_library: Res<AssetLibrary>,
    audio: Res<Audio>,
) {
    for mut bag_container in bag_query.iter_mut() {
        if bag_container.slots.len() > 0
            && bag_container
                .slots
                .get(bag_container.slots.len() - 1)
                .unwrap()
                .product_entity
                .is_some()
        {
            for slot in bag_container.slots.iter_mut() {
                if let Some(product_entity) = slot.product_entity {
                    commands.entity(product_entity).despawn_recursive();
                }
                slot.product_entity = None;
            }
            if bag_container.valid_stack() {
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
            } else {
                health_damage_events.send_default();
            }
            bag_container.products = vec![];
        }
    }
}
