use bevy::prelude::*;

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameInput>()
            .add_system_to_stage(CoreStage::PreUpdate, game_input_update);
    }
}

#[derive(Default)]
pub struct GameInput {
    drag_id: u64,
    cursor_position: Option<Vec2>,
    drags: Vec<GameInputDrag>,
}

impl GameInput {
    fn next_drag_id(&mut self) -> u64 {
        let next_id = self.drag_id;
        self.drag_id = self.drag_id.wrapping_add(1);
        next_id
    }

    pub fn cursor_position(&self) -> Option<Vec2> {
        self.cursor_position
    }

    pub fn drag_started(&self) -> Option<&GameInputDrag> {
        for drag in self.drags.iter() {
            if drag.started {
                return Some(drag);
            }
        }
        None
    }

    pub fn drag_position(&self, id: u64) -> Option<Vec2> {
        if let Some(drag) = self.drags.iter().find(|drag| drag.id == id) {
            Some(drag.position)
        } else {
            None
        }
    }

    pub fn drag_ended(&self, id: u64) -> bool {
        if let Some(drag) = self.drags.iter().find(|drag| drag.id == id) {
            drag.ended
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct GameInputDrag {
    id: u64,
    source: GameInputDragSource,
    position: Vec2,
    started: bool,
    ended: bool,
}

impl GameInputDrag {
    pub fn new(id: u64, source: GameInputDragSource, position: Vec2) -> Self {
        Self {
            id,
            source,
            position,
            started: true,
            ended: false,
        }
    }
}

impl GameInputDrag {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameInputDragSource {
    Cursor,
    Touch(u64),
}

fn game_input_update(
    mut game_input: ResMut<GameInput>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_buttons: Res<Input<MouseButton>>,
    touches: Res<Touches>,
) {
    let window_size = if let Some(window) = windows.get_primary() {
        Vec2::new(window.width() as f32, window.height() as f32)
    } else {
        Vec2::new(1440., 810.)
    };
    let to_world_matrix = if let Some((camera, camera_transform)) = camera_query.get_single().ok() {
        camera_transform.compute_matrix() * camera.projection_matrix().inverse()
    } else {
        Mat4::IDENTITY
    };
    #[cfg(target_os = "ios")]
    let world_correction = Vec2::ONE;
    #[cfg(not(target_os = "ios"))]
    let world_correction = Vec2::new(1., -1.);
    let to_world = move |position: Vec2| {
        to_world_matrix
            .project_point3(((position / window_size) * 2.0 - Vec2::ONE).extend(-1.0))
            .truncate()
            * world_correction
    };

    game_input.cursor_position = None;
    if let Some(window) = windows.get_primary() {
        if let Some(cursor_position) = window.cursor_position() {
            game_input.cursor_position = Some(to_world(cursor_position) * Vec2::new(1., -1.));
        }
    }

    for drag in game_input.drags.iter_mut() {
        if drag.started {
            drag.started = false;
            break;
        }
    }
    game_input.drags.retain(|drag| !drag.ended);

    let mut has_touch = false;
    for touch in touches.iter() {
        if touches.just_pressed(touch.id()) {
            let id = game_input.next_drag_id();
            game_input.drags.push(GameInputDrag::new(
                id,
                GameInputDragSource::Touch(touch.id()),
                to_world(touch.position()),
            ));
        } else {
            if let Some(touch_drag) = game_input
                .drags
                .iter_mut()
                .find(|drag| drag.source == GameInputDragSource::Touch(touch.id()))
            {
                touch_drag.position = to_world(touch.position());
            }
        }
        has_touch = true;
    }
    for touch in touches.iter_just_released() {
        if let Some(touch_drag) = game_input
            .drags
            .iter_mut()
            .find(|drag| drag.source == GameInputDragSource::Touch(touch.id()))
        {
            touch_drag.ended = true;
        }
    }
    for touch in touches.iter_just_cancelled() {
        if let Some(touch_drag) = game_input
            .drags
            .iter_mut()
            .find(|drag| drag.source == GameInputDragSource::Touch(touch.id()))
        {
            touch_drag.ended = true;
        }
    }

    if let Some(cursor_position) = game_input.cursor_position {
        if mouse_buttons.just_pressed(MouseButton::Left) && !has_touch {
            let id = game_input.next_drag_id();
            game_input.drags.push(GameInputDrag::new(
                id,
                GameInputDragSource::Cursor,
                cursor_position,
            ));
        } else {
            if let Some(mouse_drag) = game_input
                .drags
                .iter_mut()
                .find(|drag| drag.source == GameInputDragSource::Cursor)
            {
                mouse_drag.position = cursor_position;
            }
        }
    }
    if mouse_buttons.just_released(MouseButton::Left) || has_touch {
        if let Some(mouse_drag) = game_input
            .drags
            .iter_mut()
            .find(|drag| drag.source == GameInputDragSource::Cursor)
        {
            mouse_drag.ended = true;
        }
    }
}
