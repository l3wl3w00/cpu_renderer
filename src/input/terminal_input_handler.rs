use crate::input::InputAction::{ActionOnScene, ChangeRenderType};
use crate::input::SceneAction::RotateCamera;
use crate::input::{InputAction, InputHandler, MoveDirection, SceneAction};
use crate::image::Image;
use cgmath::Vector2;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, MouseEvent, MouseEventKind};
use std::collections::HashSet;
use std::time::Duration;

pub struct TerminalInputHandler {
    toggled_actions: HashSet<InputAction>,
    single_time_actions: HashSet<InputAction>,
    last_mouse_pos: Option<(u16, u16)>,
}

impl InputHandler for TerminalInputHandler {
    fn contains_input(&self, input_action: InputAction) -> bool {
        self.toggled_actions.contains(&input_action) ||
        self.single_time_actions.contains(&input_action)
    }

    fn input_actions(&self) -> impl Iterator<Item=InputAction> + '_ {
        Iterator::chain(
            self.toggled_actions.iter(),
            self.single_time_actions.iter())
            .map(|action| *action)
    }

    fn scene_actions(&self) -> impl Iterator<Item=SceneAction>  + '_ {
        self.input_actions()
            .filter_map(|action|
                if let ActionOnScene(action) = action {
                    Some(action)
                } else {
                    None
                }
            )
    }

    fn poll_event(&mut self, dt: &Duration) -> std::io::Result<()> {
        self.single_time_actions.clear();
        if !event::poll(*dt)? {
            return Ok(());
        }

        match event::read()? {
            Event::Key(key_event) => self.handle_key_event(key_event),
            Event::Mouse(mouse_event) => {
                self.handle_mouse_event(mouse_event, self.last_mouse_pos);
                self.last_mouse_pos = Some((mouse_event.row, mouse_event.column));
            }
            _ => {}
        };

        Ok(())
    }
}

impl TerminalInputHandler {
    pub fn new() -> Self {
        Self {
            toggled_actions: HashSet::new(),
            single_time_actions: HashSet::new(),
            last_mouse_pos: None,
        }
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent, last_mouse_pos: Option<(u16, u16)>) {
        if mouse_event.kind != MouseEventKind::Moved {
            return;
        }

        if let Some((last_x, last_y)) = last_mouse_pos {
            let last_mouse_pos_screen_space = pixel_to_screen_space(last_x, last_y);
            let current_mouse_pos_screen_space = pixel_to_screen_space(mouse_event.row, mouse_event.column);
            let delta = (last_mouse_pos_screen_space - current_mouse_pos_screen_space) * 1000.;
            let delta_int = Vector2::new(delta.x.round() as i16, delta.y.round() as i16);
            let action = ActionOnScene(RotateCamera {
                delta: delta_int
            });
            self.single_time_actions.insert(action);
        }
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.code == KeyCode::Tab && key_event.kind == KeyEventKind::Press {
            self.single_time_actions.insert(ChangeRenderType);
            return;
        }

        match key_event.kind {
            KeyEventKind::Press => {
                if let Some(action) = self.map_key_to_action(key_event.code) {
                    self.toggled_actions.insert(action);
                }
            }
            KeyEventKind::Release => {
                if let Some(action) = self.map_key_to_action(key_event.code) {
                    self.toggled_actions.remove(&action);
                }
            }
            _ => { }
        }
    }

    fn map_key_to_action(&self, key_code: KeyCode) -> Option<InputAction> {
        if key_code == KeyCode::Esc {
            return Some(InputAction::Quit)
        }

        let char_key = if let KeyCode::Char(key) = key_code {
            key
        } else {
            return None;
        };

        let move_dir = match char_key {
            'w' => Some(MoveDirection::Forward),
            'a' => Some(MoveDirection::Left),
            's' => Some(MoveDirection::Backward),
            'd' => Some(MoveDirection::Right),
            'q' => Some(MoveDirection::Down),
            'e' => Some(MoveDirection::Up),
            _ => None
        };
        move_dir.map(|dir| {ActionOnScene(SceneAction::Move(dir))})
    }
}

pub fn pixel_to_screen_space(row: u16, col: u16) -> Vector2<f32> {
    let pixel_x_middle = row as f32 + 0.5;
    let pixel_y_middle = col as f32 + 0.5;

    let x_normalized = pixel_x_middle / Image::width() as f32;
    let y_normalized = pixel_y_middle / Image::height() as f32;

    let x_screen = 2.0 * x_normalized - 1.0;
    let y_screen = 1.0 - 2.0 * y_normalized;

    Vector2::new(x_screen, y_screen)
}