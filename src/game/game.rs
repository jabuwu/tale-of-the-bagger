use bevy::prelude::*;
use lerp::Lerp;
use rand::{seq::IteratorRandom, thread_rng};
use strum::IntoEnumIterator;

use crate::{common::Transform2, AppState, AssetLibrary};

use super::{
    BagPlugin, BagSpawnEvent, BagSystem, ContainerPlugin, ConveyorPlugin, CustomerPlugin,
    CustomerSpawnEvent, DeskPlugin, DeskSpawnEvent, HealthIconSpawnEvent, HealthPlugin,
    ProductKind, ProductPlugin, ProductSpawnEvent, DEPTH_BACKGROUND, DEPTH_BACKGROUND_FRONT,
};

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DeskPlugin)
            .add_plugin(ConveyorPlugin)
            .add_plugin(BagPlugin)
            .add_plugin(CustomerPlugin)
            .add_plugin(ProductPlugin)
            .add_plugin(ContainerPlugin)
            .add_plugin(HealthPlugin)
            .add_system_set(
                SystemSet::on_enter(AppState::Game)
                    .with_system(game_enter)
                    .before(BagSystem::Spawn),
            )
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(game_spawn_customers))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(game_spawn_products))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(game_esc_to_menu));

        #[cfg(not(feature = "dev"))]
        app.add_plugin(super::AmbiencePlugin);
    }
}

fn game_enter(
    mut commands: Commands,
    mut desk_spawn_events: EventWriter<DeskSpawnEvent>,
    mut bag_spawn_events: EventWriter<BagSpawnEvent>,
    mut health_spawn_events: EventWriter<HealthIconSpawnEvent>,
    asset_library: Res<AssetLibrary>,
) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            texture: asset_library.textures.background.clone(),
            ..Default::default()
        })
        .insert(Transform2::from_xy(0., 0.))
        .insert(DEPTH_BACKGROUND);
    commands
        .spawn(SpriteBundle {
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

    for i in 0..4 {
        let x = 870. - i as f32 * 90.;
        health_spawn_events.send(HealthIconSpawnEvent {
            position: Vec2::new(x, 450.),
            threshold: i + 1,
        });
    }
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
            entity: commands.spawn_empty().id(),
            position: Vec2::new(-2000., -100.),
            kind: ProductKind::iter().choose(&mut thread_rng()).unwrap(),
        });
        local.spawn_time = 1.5;
    }
}

fn game_esc_to_menu(mut app_state: ResMut<State<AppState>>, mut input: ResMut<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        let _ = app_state.set(AppState::Menu);
        input.reset(KeyCode::Escape);
    }
}
