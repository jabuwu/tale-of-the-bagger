use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::AssetLibrary;

use super::Conveyor;

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
            .add_system(desk_spawned.label(DeskSystem::Spawned));
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
                transform: Transform::from_xyz(-105., -256.5, 0.2).with_scale(Vec3::splat(0.75)),
                ..Default::default()
            })
            .insert(SpineSync)
            .insert(Desk);
    }
}

fn desk_spawned(mut spine_ready_event: EventReader<SpineReadyEvent>, mut commands: Commands) {
    for event in spine_ready_event.iter() {
        commands
            .entity(*event.bones.get("conveyor").unwrap())
            .insert(Conveyor::default());
    }
}
