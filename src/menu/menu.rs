use bevy::{app::AppExit, prelude::*};
use bevy_spine::prelude::*;

use crate::{
    common::{Aabb, CollisionShape, GameInput, Interactable, Transform2},
    AppState, AssetLibrary,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum MenuSystem {
    Spawn,
    Spawned,
    ButtonUpdate,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MenuSpawnEvent>()
            .add_system(menu_spawn.label(MenuSystem::Spawn))
            .add_system(menu_spawned.label(MenuSystem::Spawned))
            .add_system(menu_button_update.label(MenuSystem::ButtonUpdate));
    }
}

#[derive(Default)]
pub struct MenuSpawnEvent;

#[derive(Component)]
struct Menu;

#[derive(Component)]
struct MenuButton {
    kind: MenuButtonKind,
    slot: SlotHandle,
}

#[derive(Component)]
enum MenuButtonKind {
    Play,
    Help,
    Quit,
}

fn menu_spawn(
    mut spawn_events: EventReader<MenuSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in spawn_events.iter() {
        commands
            .spawn(SpineBundle {
                skeleton: asset_library.spines.menu.clone(),
                ..Default::default()
            })
            .insert(Transform2::default())
            .insert(Menu);
    }
}

fn menu_spawned(
    mut spine_ready_event: EventReader<SpineReadyEvent>,
    mut commands: Commands,
    mut menu_query: Query<&mut Spine, &Menu>,
) {
    for event in spine_ready_event.iter() {
        if let Some(menu_spine) = menu_query.get_mut(event.entity).ok() {
            commands.entity(event.entity).with_children(|parent| {
                for (slot_name, bounds_name, kind) in [
                    ("play", "play_bounds", MenuButtonKind::Play),
                    ("help", "help_bounds", MenuButtonKind::Help),
                    ("quit", "quit_bounds", MenuButtonKind::Quit),
                ] {
                    let slot = menu_spine.skeleton.find_slot(slot_name).unwrap().handle();
                    let bounds = Aabb::new_from_vertices(
                        &menu_spine
                            .skeleton
                            .find_slot(bounds_name)
                            .unwrap()
                            .bounding_box_attachment()
                            .unwrap()
                            .vertices2()
                            .iter()
                            .map(|vec| Vec2::from(*vec))
                            .collect::<Vec<Vec2>>(),
                    )
                    .unwrap();
                    parent
                        .spawn(TransformBundle::default())
                        .insert(Transform2::from_translation(bounds.translation))
                        .insert(Interactable::new(
                            CollisionShape::Aabb {
                                half_extents: bounds.half_extents,
                            },
                            Vec2::ZERO,
                        ))
                        .insert(MenuButton { kind, slot });
                }
            });
        }
    }
}

fn menu_button_update(
    mut spine_query: Query<&mut Spine, With<Spine>>,
    mut app_state: ResMut<State<AppState>>,
    mut exit_events: EventWriter<AppExit>,
    menu_button_query: Query<(&MenuButton, &Interactable, &Parent)>,
    game_input: Res<GameInput>,
) {
    for (menu_button, menu_button_interactable, menu_button_parent) in menu_button_query.iter() {
        if let Some(mut menu_spine) = spine_query.get_mut(menu_button_parent.get()).ok() {
            menu_button
                .slot
                .get_mut(&mut menu_spine.skeleton)
                .unwrap()
                .color_mut()
                .a = if menu_button_interactable.hovered(game_input.as_ref()) {
                1.
            } else {
                0.
            };
            if menu_button_interactable
                .drag_started(game_input.as_ref())
                .is_some()
            {
                match menu_button.kind {
                    MenuButtonKind::Play => {
                        let _ = app_state.set(AppState::Game);
                    }
                    MenuButtonKind::Help => {}
                    MenuButtonKind::Quit => {
                        exit_events.send_default();
                    }
                }
            }
        }
    }
}
