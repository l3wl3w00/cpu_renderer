use crate::core::scene::Scene;
use cgmath::Vector3;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use crate::input::terminal_input_handler::pixel_to_screen_space;

const WIDTH_TO_HEIGHT_RATIO: f32 = 16. / 9.;
const SCREEN_HEIGHT: usize = 120;
const SCREEN_WIDTH: usize = (SCREEN_HEIGHT as f32 * WIDTH_TO_HEIGHT_RATIO) as usize;
pub const PIXEL_COUNT: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

pub struct Pixel { color: Vector3<f32> }
impl Pixel {
    pub fn new(color: Vector3<f32>) -> Pixel {
        Pixel {color}
    }
    pub fn intensity(&self) -> f32 {
        (self.color.x + self.color.y + self.color.z) / 3.0
    }

    pub fn color(&self) -> (u8,u8,u8) {
        (
            (self.color.x * 255.).round() as u8,
            (self.color.y * 255.).round() as u8,
            (self.color.z * 255.).round() as u8,
        )
    }
}
pub(crate) struct Image {
    pixels: [Pixel; PIXEL_COUNT],
}

impl Image {
    pub fn write(&mut self, scene: &Scene) {
        self.pixels
            .par_iter_mut()
            .enumerate()
            .map(|(i, p)| {
                let col = i % Self::width();
                let row = i / Self::width();
                let pixel_in_screen_space = pixel_to_screen_space(col as u16, row as u16);
                let ray = scene.camera().get_ray(pixel_in_screen_space);
                (p, scene.trace(&ray))
            })
            .for_each(|(p, color)| *p = Pixel::new(color));
    }

    pub fn new(pixels: [f32; PIXEL_COUNT]) -> Image {
        Image {
            pixels: pixels.map(|p| Pixel::new(Vector3::new(p, 0., 0.))),
        }
    }
    pub fn pixels(&self) -> &[Pixel; PIXEL_COUNT] {
        &self.pixels
    }

    pub fn width() -> usize {
        SCREEN_WIDTH
    }

    pub(crate) fn height() -> usize {
        SCREEN_HEIGHT
    }
}
