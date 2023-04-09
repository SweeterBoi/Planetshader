#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

extern crate sdl2;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use sdl2::rect::Rect;

use std::time::Duration;

mod dataStructures;
mod background;
mod planet;
mod converter;
mod test;
mod landmass;
mod atmoshphere;

use crate::{dataStructures::{Vect2D, Vect3D, ColorScheme}, background::BackgroundStars, planet::Planet};


const SCREENWIDTH: u32 = 800;
const SCREENHEIGHT: u32 = 600;

pub fn main() -> Result<(), String> {
    let sdl_context: sdl2::Sdl = sdl2::init()?;
    let video_subsystem: sdl2::VideoSubsystem = sdl_context.video()?;

    let window: sdl2::video::Window = video_subsystem
        .window("Planets owo", SCREENWIDTH, SCREENHEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump: sdl2::EventPump = sdl_context.event_pump()?;

    // Background stars
    let background: BackgroundStars = BackgroundStars::new(100u32);
    // ColorScheme
    let colors: ColorScheme = ColorScheme{
        secondary: Color::RGB(59, 125, 79),
        primary: Color::RGB(99, 171, 63),
        tertiary: Color::RGB(47, 87, 83),
        quaternary: Color::RGB(79, 164, 184)
    };
    //Planet
    let mut Planet: Planet = Planet::new(colors);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.present();
        // Background:
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        ::std::thread::sleep(Duration::new(0, 1_000_000u32 / 30));

        background.draw(&mut canvas);
        Planet.update();
        Planet.draw(&mut canvas);
    }

    Ok(())
}