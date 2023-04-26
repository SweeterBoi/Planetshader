#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

extern crate sdl2;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use sdl2::rect::Rect;

use std::time::{Duration, Instant};

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
const PLANETSIZE: i16 = 200i16;
// Main Function
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
    let mut background: BackgroundStars = BackgroundStars::new(100u32);
    // ColorScheme
    let mut colors: Vec<Color> = Vec::new();
    colors.push(Color::RGB(99, 171, 63));
    colors.push(Color::RGB(59, 125, 79));
    colors.push(Color::RGB(47, 87, 83));
    colors.push(Color::RGB(40, 53, 64));
    colors.push(Color::RGB(79, 164, 184));
    colors.push(Color::RGB(64, 73, 115));
    colors.push(Color::RGB(245, 255, 232));
    colors.push(Color::RGB(223, 224, 232));
    colors.push(Color::RGB(104, 111, 153));
    colors.push(Color::RGB(64, 73, 115));
    //Planet
    let mut Planet: Planet = Planet::new(colors.clone(), PLANETSIZE);
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space), ..
                } => {Planet = Planet::new(colors.clone(), PLANETSIZE);}
                _ => {}
            }
        }
        canvas.present();
        // Background:
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        //::std::thread::sleep(Duration::new(0, 1_000_000u32 / 30));
        background.draw(&mut canvas);
        Planet.update();
        Planet.draw(&mut canvas);
        //drawColorScheme(&mut canvas, &colors);
    }

    Ok(())
}

fn drawColorScheme(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, colors: &Vec<Color>) {
    let topLeftCorner: Vect2D = Vect2D {x : 700f64, y: 10f64};
    let space: f64 = SCREENHEIGHT as f64 / colors.len() as f64;
    let size: u32 = (2.0 * space / 3.0) as u32;
    let margin: u32 = (space / 3.0) as u32;
    for (i, color) in colors.iter().enumerate() {
        let yi: u32 = topLeftCorner.y as u32 + (i as u32 * (size + margin));
        canvas.set_draw_color(*color);
        match canvas.fill_rect(Rect::new(topLeftCorner.x as i32, yi as i32, size, size)){
            Ok(_) => (),
            Err(_) => {}
            };
        
    }
}
