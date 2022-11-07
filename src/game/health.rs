use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_spine::prelude::*;

use crate::{
    common::{SpineSync2, Transform2},
    AssetLibrary,
};

use super::DEPTH_HEALTH;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum HealthSystem {
    Damage,
    IconSpawn,
    IconUpdate,
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Health>()
            .add_event::<HealthDamageEvent>()
            .add_event::<HealthIconSpawnEvent>()
            .add_system(health_damage.label(HealthSystem::Damage))
            .add_system(health_icon_spawn.label(HealthSystem::IconSpawn))
            .add_system(health_icon_update.label(HealthSystem::IconUpdate));
    }
}

pub struct Health {
    pub amount: u8,
}

impl Default for Health {
    fn default() -> Self {
        Self { amount: 4 }
    }
}

#[derive(Default)]
pub struct HealthDamageEvent;

#[derive(Default)]
pub struct HealthIconSpawnEvent {
    pub position: Vec2,
    pub threshold: u8,
}

#[derive(Component)]
pub struct HealthIcon {
    pub threshold: u8,
    pub lost: bool,
}

fn health_damage(
    mut damage_events: EventReader<HealthDamageEvent>,
    mut health: ResMut<Health>,
    input: Res<Input<KeyCode>>,
    asset_library: Res<AssetLibrary>,
    audio: Res<Audio>,
) {
    for _ in damage_events.iter() {
        if health.amount > 0 {
            audio.play(asset_library.audio.bag_clear_error.clone());
            health.amount -= 1;
        }
    }
    if input.just_pressed(KeyCode::H) {
        if health.amount > 0 {
            audio.play(asset_library.audio.bag_clear_error.clone());
            health.amount -= 1;
        }
    }
}

fn health_icon_spawn(
    mut spawn_events: EventReader<HealthIconSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        commands
            .spawn_bundle(SpineBundle {
                skeleton: asset_library.spines.health.clone(),
                ..Default::default()
            })
            .insert(Transform2::from_translation(event.position))
            .insert(DEPTH_HEALTH)
            .insert(SpineSync2)
            .insert(HealthIcon {
                threshold: event.threshold,
                lost: false,
            });
    }
}

fn health_icon_update(mut health_query: Query<(&mut HealthIcon, &mut Spine)>, health: Res<Health>) {
    for (mut health_icon, mut health_icon_spine) in health_query.iter_mut() {
        if !health_icon.lost && health.amount < health_icon.threshold {
            let _ = health_icon_spine
                .animation_state
                .set_animation_by_name(0, "lose", false);
            health_icon.lost = true;
        }
    }
}
