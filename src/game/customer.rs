use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::{SpineSync2, Transform2},
    AssetLibrary,
};

use super::DEPTH_CUSTOMER;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum CustomerSystem {
    Spawn,
    Spawned,
    Update,
}

pub struct CustomerPlugin;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CustomerSpawnEvent>()
            .add_system(customer_spawn.label(CustomerSystem::Spawn))
            .add_system(customer_spawned.label(CustomerSystem::Spawned))
            .add_system(customer_update.label(CustomerSystem::Update));
    }
}

#[derive(Default)]
pub struct CustomerSpawnEvent {
    pub position: Vec2,
}

#[derive(Component)]
pub struct Customer;

fn customer_spawn(
    mut spawn_events: EventReader<CustomerSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        commands
            .spawn_bundle(SpineBundle {
                skeleton: asset_library.spines.customer.clone(),
                ..Default::default()
            })
            .insert(Transform2::from_translation(event.position).with_scale(Vec2::splat(0.75)))
            .insert(DEPTH_CUSTOMER)
            .insert(SpineSync2)
            .insert(Customer);
    }
}

fn customer_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut spine_query: Query<&mut Spine, With<Customer>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(mut spine) = spine_query.get_mut(event.entity).ok() {
            let _ = spine
                .animation_state
                .set_animation_by_name(0, "animation", true);
        }
    }
}

fn customer_update(
    mut customer_query: Query<(Entity, &mut Transform2), With<Customer>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (customer_entity, mut customer_transform) in customer_query.iter_mut() {
        customer_transform.translation.x += time.delta_seconds() * 100.;
        if customer_transform.translation.x > 800. {
            commands.entity(customer_entity).despawn_recursive();
        }
    }
}
