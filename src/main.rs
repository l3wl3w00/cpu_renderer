#![feature(iterator_try_collect)]
use crate::core::camera::Camera;
use crate::core::light::Light;
use crate::core::scene::Scene;
use crate::core::screen::Image;
use crate::core::shapes::Sphere;
use cgmath::{Vector3, Zero};
use rand::random;
use std::time::Duration;
use crate::core::screen;
use crate::input::{InputAction, InputHandler};
use crate::input::terminal_input_handler::TerminalInputHandler;
use crate::render::{Renderer};
use crate::render::terminal_renderer::{TerminalRenderType, TerminalRenderer};

mod core;
mod render;
mod input;

fn main() -> std::io::Result<()> {
    let mut screen_image = Image::new([0.0; screen::PIXEL_COUNT]);
    let mut renderer = TerminalRenderer::new(TerminalRenderType::Colored);
    let mut input_handler = TerminalInputHandler::new();

    run_game(&mut input_handler, &mut renderer, &mut screen_image)?;

    Ok(())
}
fn run_game(
    input_handler: &mut impl InputHandler,
    renderer: &mut impl Renderer,
    screen_image: &mut Image
) -> std::io::Result<()> {
    let mut scene = create_scene();

    let dt = Duration::from_millis(16);
    let mut total_time = Duration::from_millis(0);

    loop {
        input_handler.poll_event(&dt)?;
        if input_handler.input_actions().any(|a| a == InputAction::Quit) {
            break Ok(());
        }

        total_time += dt;
        scene.tick(input_handler.scene_actions(), &dt, &total_time);
        renderer.tick(input_handler.input_actions());

        screen_image.write(&scene);

        renderer.render(&screen_image);
    }
}
fn create_scene() -> Scene {
    let camera = Camera::from_position_and_target(Vector3::zero(), -Vector3::unit_z());
    let mut scene = Scene::new(camera);
    let x_range: (f32, f32) = (-5., 5.);
    let y_range: (f32, f32) = (-5., 5.);
    let z_range: (f32, f32) = (-5., 5.);
    let radius_range: (f32, f32) = (0.75, 1.5);

    scene.add_sphere(Sphere::new(Vector3::new(0.0, 0.0, -6.0), 0.1, Vector3::new(1., 1., 1.)));
    scene.add_light(Light::from_position(Vector3::new(0.0, 1.0, -5.0)));


    for _ in 0..10 {
        let x = random::<f32>() * (x_range.0 - x_range.1).abs() + x_range.0;
        let y = random::<f32>() * (y_range.0 - y_range.1).abs() + y_range.0;
        let z = random::<f32>() * (z_range.0 - z_range.1).abs() + z_range.0;
        let radius = random::<f32>() * (radius_range.0 - radius_range.1).abs() + radius_range.0;

        let r = random::<f32>();
        let g = random::<f32>();
        let b = random::<f32>();

        scene.add_sphere(Sphere::new(Vector3::new(x, y, z), radius, Vector3::new(r, g, b)));
    }

    scene
}
