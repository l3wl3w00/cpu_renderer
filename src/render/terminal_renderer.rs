use crate::core::image::Image;
use crate::input::InputAction;
use crate::render::{Renderer, PIXEL_TYPES, PIXEL_TYPE_COUNT};
use colored::Colorize;
use crossterm::{cursor, event, execute, terminal, ExecutableCommand};
use std::io::{stdout, Stdout, Write};
use crossterm::cursor::MoveTo;
use crossterm::terminal::ClearType;

pub enum TerminalRenderType {
    Colored, BlackAndWhite
}
pub struct TerminalRenderer {
    chars_buffer: String,
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
        let _ = execute!(self.stdout, MoveTo(0, 0));

        for (index, pixel) in image.pixels().iter().enumerate() {
            if index % Image::width() == 0 {
                self.chars_buffer.push_str("\n");
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
                    format!("{}", PIXEL_TYPES[index])
                }
            };
            self.chars_buffer.push_str(&c);
        }

        self.stdout.write_all(self.chars_buffer.as_bytes()).unwrap();
        self.stdout.flush().unwrap();
        self.chars_buffer.clear();
    }
}

impl TerminalRenderer {
    pub fn new(render_type: TerminalRenderType) -> TerminalRenderer {
        let mut stdout = stdout();
        terminal::enable_raw_mode().unwrap();
        stdout
            .execute(event::EnableMouseCapture)
            .expect("Terminal doesn't allow mouse capture")
            .execute(cursor::Hide)
            .expect("Terminal doesn't allow cursor hiding capture")
            .execute(terminal::Clear(ClearType::All))
            .expect("Terminal couldn't be cleared");

        TerminalRenderer {
            chars_buffer: String::new(),
            stdout,
            render_type
        }
    }
}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        let _ = self.stdout.execute(terminal::Clear(ClearType::All));
        terminal::disable_raw_mode().unwrap();
    }
}