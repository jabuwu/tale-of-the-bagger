use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::{SpineSync2, Transform2},
    AssetLibrary,
};

use super::DEPTH_BAG;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum BagSystem {
    Spawn,
}

pub struct BagPlugin;

impl Plugin for BagPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BagSpawnEvent>()
            .add_system(bag_spawn.label(BagSystem::Spawn));
    }
}

#[derive(Default)]
pub struct BagSpawnEvent {
    pub position: Vec2,
}

#[derive(Component)]
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
            .insert(Bag);
    }
}
