use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::{SpineSync2, Transform2},
    AssetLibrary,
};

use super::{Conveyor, DEPTH_DESK};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum DeskSystem {
    Spawn,
    Spawned,
}

pub struct DeskPlugin;

impl Plugin for DeskPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeskSpawnEvent>()
            .add_system(desk_spawn.label(DeskSystem::Spawn))
            .add_system(
                desk_spawned
                    .label(DeskSystem::Spawned)
                    .before_spine_sync::<SpineSync2>(),
            );
    }
}

#[derive(Default)]
pub struct DeskSpawnEvent;

#[derive(Component)]
pub struct Desk;

fn desk_spawn(
    mut spawn_events: EventReader<DeskSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in spawn_events.iter() {
        commands
            .spawn_bundle(SpineBundle {
                skeleton: asset_library.spines.desk.clone(),
                ..Default::default()
            })
            .insert(Transform2::from_xy(-140., -342.))
            .insert(DEPTH_DESK)
            .insert(SpineSync2)
            .insert(Desk);
    }
}

fn desk_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    desk_query: Query<&Desk>,
) {
    for event in spine_ready_event.iter() {
        if desk_query.contains(event.entity) {
            commands
                .entity(*event.bones.get("conveyor").unwrap())
                .insert(Conveyor::default())
                .insert(Transform2::default());
        }
    }
}
