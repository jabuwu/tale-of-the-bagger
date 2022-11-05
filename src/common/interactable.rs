use bevy::prelude::*;

use super::{CollisionShape, GameInput};

pub struct InteractablePlugin;

impl Plugin for InteractablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(interactable_update);
    }
}

#[derive(Component)]
pub struct Interactable {
    pub shape: CollisionShape,
    pub offset: Vec2,

    translation: Vec2,
}

impl Interactable {
    pub fn new(shape: CollisionShape, offset: Vec2) -> Self {
        Self {
            shape,
            offset,
            translation: Vec2::ZERO,
        }
    }

    pub fn hovered(&self, game_input: &GameInput) -> bool {
        if let Some(cursor_position) = game_input.cursor_position() {
            self.shape.colliding(
                self.translation + self.offset,
                &CollisionShape::Point,
                cursor_position,
            )
        } else {
            false
        }
    }

    pub fn contains_point(&self, point: Vec2) -> bool {
        self.shape.colliding(
            self.translation + self.offset,
            &CollisionShape::Point,
            point,
        )
    }

    pub fn drag_started(&self, game_input: &GameInput) -> Option<u64> {
        if let Some(drag) = game_input.drag_started() {
            if self.shape.colliding(
                self.translation + self.offset,
                &CollisionShape::Point,
                drag.position(),
            ) {
                Some(drag.id())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn dragging_within(&self, game_input: &GameInput, drag_id: u64) -> bool {
        if let Some(drag_position) = game_input.drag_position(drag_id) {
            self.contains_point(drag_position)
        } else {
            false
        }
    }
}

fn interactable_update(mut interactable_query: Query<(&mut Interactable, &GlobalTransform)>) {
    for (mut interactable, interactable_transform) in interactable_query.iter_mut() {
        interactable.translation = interactable_transform.translation().truncate();
    }
}
