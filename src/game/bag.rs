use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::{Aabb, CollisionShape, Hoverable, SpineSync2, Transform2},
    AssetLibrary,
};

use super::DEPTH_BAG;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum BagSystem {
    Spawn,
    Spawned,
    Update,
    Hover,
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
            );
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
    spine_query: Query<(Entity, &Spine), With<Bag>>,
) {
    for event in spine_ready_event.iter() {
        if let Some((spine_entity, spine)) = spine_query.get(event.entity).ok() {
            if let Some(bounds) = spine
                .skeleton
                .find_slot("bounds")
                .unwrap()
                .bounding_box_attachment()
            {
                let aabb_bounds = Aabb::new_from_vertices(
                    &bounds
                        .vertices2()
                        .iter()
                        .map(|vec| Vec2::from(*vec))
                        .collect::<Vec<Vec2>>(),
                )
                .unwrap();
                commands.entity(spine_entity).insert(Hoverable::new(
                    CollisionShape::Aabb {
                        half_extents: aabb_bounds.half_extents,
                    },
                    aabb_bounds.translation,
                ));
            }
        }
    }
}

fn bag_update(mut bag_query: Query<(&mut Spine, &Hoverable), With<Bag>>) {
    for (mut bag_spine, bag_hoverable) in bag_query.iter_mut() {
        *bag_spine.skeleton.find_slot_mut("bag").unwrap().color_mut() = if bag_hoverable.hovered {
            bevy_spine::Color::new_rgba(1., 0., 0., 1.)
        } else {
            bevy_spine::Color::new_rgba(1., 1., 1., 1.)
        };
    }
}
