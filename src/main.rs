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

use crate::{dataStructures::{Vect2D, Vect3D}, background::{BackgroundStars}, planet::{Planet}};


const SCREENWIDTH: u32 = 800;
const SCREENHEIGHT: u32 = 600;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("sdl2 isometric demo", SCREENWIDTH, SCREENHEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let background: BackgroundStars = BackgroundStars::new(100u32);
    let mut Planet = Planet::new();

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
        // The rest of the game loop goes here...
        background.draw(&mut canvas);
        Planet.draw(&mut canvas);
    }

    Ok(())
}