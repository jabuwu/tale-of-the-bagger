use bevy::prelude::*;

use super::{CollisionShape, Cursor};

pub struct HoverablePlugin;

impl Plugin for HoverablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(hoverable_update);
    }
}

#[derive(Component)]
pub struct Hoverable {
    pub shape: CollisionShape,
    pub offset: Vec2,
    pub hovered: bool,
}

impl Hoverable {
    pub fn new(shape: CollisionShape, offset: Vec2) -> Self {
        Self {
            shape,
            offset,
            hovered: false,
        }
    }
}

fn hoverable_update(
    mut hoverable_query: Query<(&mut Hoverable, &GlobalTransform)>,
    cursor: Res<Cursor>,
) {
    for (mut hoverable, hoverable_transform) in hoverable_query.iter_mut() {
        hoverable.hovered = hoverable.shape.colliding(
            hoverable_transform.translation().truncate() + hoverable.offset,
            &CollisionShape::Point,
            cursor.position,
        );
    }
}
