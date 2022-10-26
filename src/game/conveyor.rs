use bevy::prelude::*;
use bevy_spine::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ConveyorSystem {
    Update,
}

pub struct ConveyorPlugin;

impl Plugin for ConveyorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            conveyor_update
                .label(ConveyorSystem::Update)
                .during_spine_sync::<SpineSync>(),
        );
    }
}

#[derive(Default, Component)]
pub struct Conveyor {
    x: f32,
}

fn conveyor_update(mut conveyor_query: Query<(&mut Conveyor, &mut Transform)>, time: Res<Time>) {
    for (mut conveyor, mut conveyor_transform) in conveyor_query.iter_mut() {
        conveyor.x = (conveyor.x + time.delta_seconds() * 100.) % 1566.811;
        conveyor_transform.translation.x = conveyor.x - 16.111;
    }
}
