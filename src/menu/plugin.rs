use bevy::prelude::*;

use crate::AppState;

use super::{MenuPlugin, MenuSpawnEvent, MenuSystem};

pub struct MenuStatePlugin;

impl Plugin for MenuStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MenuPlugin).add_system_set(
            SystemSet::on_enter(AppState::Menu)
                .with_system(menu_enter)
                .before(MenuSystem::Spawn),
        );
    }
}

fn menu_enter(mut commands: Commands, mut menu_spawn_events: EventWriter<MenuSpawnEvent>) {
    commands.spawn(Camera2dBundle::default());
    menu_spawn_events.send_default();
}
