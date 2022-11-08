use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::AppState;

pub struct ClearScenePlugin;

impl Plugin for ClearScenePlugin {
    fn build(&self, app: &mut App) {
        for state in AppState::iter() {
            app.add_system_set(SystemSet::on_exit(state).with_system(clear_scene));
        }
    }
}

#[derive(Component)]
pub struct Persistent;

fn clear_scene(
    mut commands: Commands,
    query: Query<Entity, (Without<Persistent>, Without<Parent>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
