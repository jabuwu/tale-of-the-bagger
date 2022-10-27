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
    camera: Query<(&Camera, &GlobalTransform)>,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    game_input.cursor_position = None;
    if let Some(window) = windows.get_primary() {
        if let Some(position) = window.cursor_position() {
            if let Ok((camera, camera_transform)) = camera.get_single() {
                let window_size = Vec2::new(window.width() as f32, window.height() as f32);
                let ndc = (position / window_size) * 2.0 - Vec2::ONE;
                let ndc_to_world =
                    camera_transform.compute_matrix() * camera.projection_matrix().inverse();
                let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
                let world_pos: Vec2 = world_pos.truncate();
                game_input.cursor_position = Some(world_pos);
            }
        }
    }

    for drag in game_input.drags.iter_mut() {
        if drag.started {
            drag.started = false;
            break;
        }
    }
    game_input.drags.retain(|drag| !drag.ended);

    if let Some(cursor_position) = game_input.cursor_position {
        if mouse_buttons.just_pressed(MouseButton::Left) {
            let id = game_input.next_drag_id();
            game_input.drags.push(GameInputDrag {
                id,
                source: GameInputDragSource::Cursor,
                position: cursor_position,
                started: true,
                ended: false,
            });
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
    if mouse_buttons.just_released(MouseButton::Left) {
        if let Some(mouse_drag) = game_input
            .drags
            .iter_mut()
            .find(|drag| drag.source == GameInputDragSource::Cursor)
        {
            mouse_drag.ended = true;
        }
    }
}
