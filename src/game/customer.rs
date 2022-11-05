use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::{SpineSync2, Transform2},
    AssetLibrary,
};

use super::{DEPTH_CUSTOMER, DEPTH_CUSTOMER_SILHOUETTE};

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
    pub scale: f32,
    pub speed: f32,
    pub silhouette: bool,
}

#[derive(Component)]
pub struct Customer {
    speed: f32,
    silhouette: bool,
}

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
            .insert(
                Transform2::from_translation(event.position).with_scale(Vec2::ONE * event.scale),
            )
            .insert(if event.silhouette {
                DEPTH_CUSTOMER_SILHOUETTE
            } else {
                DEPTH_CUSTOMER
            })
            .insert(SpineSync2)
            .insert(Customer {
                speed: event.speed,
                silhouette: event.silhouette,
            });
    }
}

fn customer_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut spine_query: Query<(&mut Spine, &Customer)>,
) {
    for event in spine_ready_event.iter() {
        if let Some((mut spine, customer)) = spine_query.get_mut(event.entity).ok() {
            let _ = spine
                .animation_state
                .set_animation_by_name(0, "animation", true);
            if customer.silhouette {
                let _ = spine.skeleton.set_skin_by_name("silhouette");
            } else {
                let _ = spine.skeleton.set_skin_by_name("normal");
            }
        }
    }
}

fn customer_update(
    mut customer_query: Query<(Entity, &mut Transform2, &Customer)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (customer_entity, mut customer_transform, customer) in customer_query.iter_mut() {
        customer_transform.translation.x += time.delta_seconds() * customer.speed;
        if customer_transform.translation.x > 1100. {
            commands.entity(customer_entity).despawn_recursive();
        }
    }
}
