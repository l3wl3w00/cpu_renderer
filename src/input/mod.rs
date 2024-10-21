pub mod terminal_input_handler;

use cgmath::Vector2;
use std::time::Duration;

pub trait InputHandler {
    fn contains_input(&self, input_action: InputAction) -> bool;
    fn input_actions(&self) -> impl Iterator<Item=InputAction> + '_;
    fn scene_actions(&self) -> impl Iterator<Item=SceneAction> + '_;
    fn poll_event(&mut self, dt: &Duration) -> std::io::Result<()>;
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum InputAction {
    Quit,
    ChangeRenderType,
    ActionOnScene(SceneAction),
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum SceneAction {
    Move(MoveDirection),
    RotateCamera {
        // this delta value is supposed to be interpreted as follows:
        // both values of delta are guaranteed to be between 1000 and -1000
        // delta / 1000 is the mouse delta in normalized screen space
        // the reason it is not stored as a float is that this way it can be hashed
        delta: Vector2<i16>
    },
}
#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum MoveDirection {
    Right, Left, Up, Down, Forward, Backward
}