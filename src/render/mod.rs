pub mod terminal_renderer;

use crate::input::InputAction;
use crate::image::Image;

const PIXEL_TYPE_COUNT: usize = 21;
const PIXEL_TYPES: [&str; PIXEL_TYPE_COUNT] = [
    "  ",
    " .", "..",
    ".,", ",,",
    ",:", "::",
    ":;", ";;",
    ";~", "~~",
    "~=", "==",
    "=+", "++",
    "+*", "**",
    "*#", "##",
    "#@", "@@",
];
pub trait Renderer {
    fn tick(&mut self, actions: impl Iterator<Item=InputAction>);
    fn render(&mut self, image: &Image);
}