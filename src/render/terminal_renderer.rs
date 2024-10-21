use crate::core::screen::Image;
use crate::input::InputAction;
use crate::render::{Renderer, PIXEL_TYPES, PIXEL_TYPE_COUNT};
use colored::Colorize;
use crossterm::{event, ExecutableCommand};
use std::io::{stdout, Stdout, Write};
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::STD_OUTPUT_HANDLE;
use winapi::um::wincon::{SetConsoleCursorPosition, COORD};

pub enum TerminalRenderType {
    Colored, BlackAndWhite
}
pub struct TerminalRenderer {
    chars: String,
    stdout: Stdout,
    render_type: TerminalRenderType,
}
impl Renderer for TerminalRenderer {
    fn tick(&mut self, mut actions: impl Iterator<Item=InputAction>) {
        if actions.any(|a| a == InputAction::ChangeRenderType) {
            self.render_type = match self.render_type {
                TerminalRenderType::Colored => TerminalRenderType::BlackAndWhite,
                TerminalRenderType::BlackAndWhite => TerminalRenderType::Colored,
            };
        }
    }

    fn render(&mut self, image: &Image) {
        // move the cursor to the starting position,
        // so that the new image will be drawn over the old one
        unsafe {
            let handle = GetStdHandle(STD_OUTPUT_HANDLE);
            SetConsoleCursorPosition(handle, COORD { X: 0, Y: 0 });
        }

        for (index, pixel) in image.pixels().iter().enumerate() {
            if index % Image::width() == 0 {
                self.chars.push_str("\n");
            }
            let c = match self.render_type {
                TerminalRenderType::Colored => {
                    let (r, g, b) = pixel.color();
                    let c = " ".on_truecolor(r, g, b);
                    format!("{}{}", c, c)
                }
                TerminalRenderType::BlackAndWhite => {
                    let intensity = pixel.intensity();
                    let mut index = (intensity * PIXEL_TYPE_COUNT as f32).floor() as usize;
                    if index >= PIXEL_TYPE_COUNT {
                        index = PIXEL_TYPE_COUNT - 1;
                    }
                    let c = PIXEL_TYPES[index];
                    format!("{}{}", c, c)
                }
            };
            self.chars.push_str(&c);
        }

        self.stdout.write_all(self.chars.as_bytes()).unwrap();
        self.stdout.flush().unwrap();
        self.chars.clear();
    }
}

impl TerminalRenderer {
    pub fn new(render_type: TerminalRenderType) -> TerminalRenderer {
        let mut stdout = stdout();
        stdout.execute(event::EnableMouseCapture).expect("Terminal doesn't allow mouse capture");
        TerminalRenderer {
            chars: String::new(),
            stdout,
            render_type
        }
    }
}