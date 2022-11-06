use bevy::prelude::*;
use lerp::Lerp;
use rand::{seq::IteratorRandom, thread_rng};
use strum::IntoEnumIterator;

use crate::{common::Transform2, AppState, AssetLibrary};

use super::{
    BagPlugin, BagSpawnEvent, ContainerPlugin, ConveyorPlugin, CustomerPlugin, CustomerSpawnEvent,
    DeskPlugin, DeskSpawnEvent, ProductKind, ProductPlugin, ProductSpawnEvent, DEPTH_BACKGROUND,
    DEPTH_BACKGROUND_FRONT,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DeskPlugin)
            .add_plugin(ConveyorPlugin)
            .add_plugin(BagPlugin)
            .add_plugin(CustomerPlugin)
            .add_plugin(ProductPlugin)
            .add_plugin(ContainerPlugin)
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(game_enter))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(game_spawn_customers))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(game_spawn_products));

        #[cfg(not(feature = "dev"))]
        app.add_plugin(super::AmbiencePlugin);
    }
}

fn game_enter(
    mut commands: Commands,
    mut desk_spawn_events: EventWriter<DeskSpawnEvent>,
    mut bag_spawn_events: EventWriter<BagSpawnEvent>,
    asset_library: Res<AssetLibrary>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.textures.background.clone(),
            ..Default::default()
        })
        .insert(Transform2::from_xy(0., 0.))
        .insert(DEPTH_BACKGROUND);
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.textures.background_front.clone(),
            ..Default::default()
        })
        .insert(Transform2::from_xy(0., 0.))
        .insert(DEPTH_BACKGROUND_FRONT);

    desk_spawn_events.send_default();

    bag_spawn_events.send(BagSpawnEvent {
        position: Vec2::new(-700., -253.),
        ..Default::default()
    });
    bag_spawn_events.send(BagSpawnEvent {
        position: Vec2::new(-160., -253.),
        ..Default::default()
    });
    bag_spawn_events.send(BagSpawnEvent {
        position: Vec2::new(387., -253.),
        ..Default::default()
    });
}

#[derive(Default)]
struct GameSpawnCustomersLocal {
    spawn_time: f32,
}

fn game_spawn_customers(
    mut customer_spawn_events: EventWriter<CustomerSpawnEvent>,
    mut local: Local<GameSpawnCustomersLocal>,
    time: Res<Time>,
) {
    local.spawn_time -= time.delta_seconds();
    if local.spawn_time <= 0. {
        if rand::random() {
            customer_spawn_events.send(CustomerSpawnEvent {
                position: Vec2::new(-1100., 100.0_f32.lerp(200., rand::random::<f32>())),
                scale: 0.7,
                speed: 100.,
                silhouette: true,
                ..Default::default()
            });
        } else {
            customer_spawn_events.send(CustomerSpawnEvent {
                position: Vec2::new(-1100., -125.0_f32.lerp(300., rand::random::<f32>())),
                scale: 1.,
                speed: 150.,
                silhouette: false,
                ..Default::default()
            });
        }
        local.spawn_time = 17.;
    }
}

#[derive(Default)]
struct GameSpawnProductsLocal {
    spawn_time: f32,
}

fn game_spawn_products(
    mut product_spawn_events: EventWriter<ProductSpawnEvent>,
    mut local: Local<GameSpawnProductsLocal>,
    mut commands: Commands,
    time: Res<Time>,
) {
    local.spawn_time -= time.delta_seconds();
    if local.spawn_time <= 0. {
        product_spawn_events.send(ProductSpawnEvent {
            entity: commands.spawn().id(),
            position: Vec2::new(-2000., -100.),
            kind: ProductKind::iter().choose(&mut thread_rng()).unwrap(),
        });
        local.spawn_time = 1.5;
    }
}
