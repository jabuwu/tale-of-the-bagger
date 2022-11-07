use bevy::prelude::*;
use bevy_spine::prelude::*;

use crate::{
    common::SpineSync2,
    game::{ProductKind, ProductSpawnEvent},
    AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ProductBowlingBallSystem {
    Spawn,
    Spawned,
}

pub struct ProductBowlingBallPlugin;

impl Plugin for ProductBowlingBallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(product_bowling_ball_spawn.label(ProductBowlingBallSystem::Spawn))
            .add_system(product_bowling_ball_spawned.label(ProductBowlingBallSystem::Spawned));
    }
}

#[derive(Component)]
pub struct ProductBowlingBall;

#[derive(Component)]
pub struct ProductBowlingBallRig;

fn product_bowling_ball_spawn(
    mut spawn_events: EventReader<ProductSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in spawn_events.iter() {
        if event.kind == ProductKind::BowlingBall {
            commands
                .entity(event.entity)
                .insert_bundle(SpineBundle {
                    skeleton: asset_library.spines.product_bowling_ball.clone(),
                    ..Default::default()
                })
                .insert(SpineSync2)
                .insert(ProductBowlingBall);
        }
    }
}

fn product_bowling_ball_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    bowling_ball_query: Query<Entity, With<ProductBowlingBall>>,
) {
    for event in spine_ready_event.iter() {
        if let Some(bowling_ball_entity) = bowling_ball_query.get(event.entity).ok() {
            commands
                .entity(bowling_ball_entity)
                .insert(ProductBowlingBallRig);
        }
    }
}
