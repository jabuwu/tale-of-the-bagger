use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::common::{SpineSync2, Transform2};

use super::HealthDamageEvent;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ConveyorSystem {
    Update,
    ItemUpdate,
}

pub struct ConveyorPlugin;

impl Plugin for ConveyorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            conveyor_update
                .label(ConveyorSystem::Update)
                .during_spine_sync::<SpineSync2>(),
        )
        .add_system(
            conveyor_item_update
                .label(ConveyorSystem::ItemUpdate)
                .after(ConveyorSystem::Update),
        );
    }
}

#[derive(Component)]
pub struct Conveyor {
    pub x: f32,
    pub speed: f32,
}

impl Default for Conveyor {
    fn default() -> Self {
        Self { x: 0., speed: 100. }
    }
}

fn conveyor_update(mut conveyor_query: Query<(&mut Conveyor, &mut Transform2)>, time: Res<Time>) {
    for (mut conveyor, mut conveyor_transform) in conveyor_query.iter_mut() {
        conveyor.x = (conveyor.x + time.delta_seconds() * conveyor.speed) % 1566.811;
        conveyor_transform.translation.x = conveyor.x - 16.111;
    }
}

#[derive(Component, Default)]
pub struct ConveyorItem {
    pub progress: f32,
    pub position: Vec2,
}

fn conveyor_item_update(
    mut conveyor_item_query: Query<(Entity, &mut ConveyorItem)>,
    mut commands: Commands,
    mut health_damage_events: EventWriter<HealthDamageEvent>,
    conveyor_query: Query<(&Conveyor, &GlobalTransform)>,
    time: Res<Time>,
) {
    let (conveyor, conveyor_transform) =
        if let Some((conveyor, conveyor_transform)) = conveyor_query.get_single().ok() {
            (conveyor, conveyor_transform)
        } else {
            return;
        };
    for (conveyor_entity, mut conveyor_item) in conveyor_item_query.iter_mut() {
        let (conveyor_scale, _, _) = conveyor_transform.to_scale_rotation_translation();
        conveyor_item.progress += (time.delta_seconds() * conveyor.speed) * conveyor_scale.x;
        conveyor_item.position = Vec2::new(-1066. + conveyor_item.progress, -387.);
        if conveyor_item.progress > 1666. {
            commands.entity(conveyor_entity).despawn_recursive();
            health_damage_events.send_default();
        }
    }
}
