use rand::Rng;
use sdl2::sys::ScreenSaverActive;
use sdl2::{pixels::Color, gfx::primitives::DrawRenderer};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::{Rect, Point};

use noise::{Perlin, NoiseFn, Seedable};

use crate::{SCREENWIDTH, SCREENHEIGHT};
use crate::dataStructures::{Vect2D, ColorScheme};
use crate::converter::converter::PixelCoordinatesToSphereCoordinates;
use std::f64::consts::PI;

pub struct LandMass {
    position: Vect2D,
    size: i16,
    seed: u32,
    rotationalAngle: f64,
}

/*
    Implementation of the LandMass struct
 */
impl LandMass {
    /*
        Instantiates a new LandMass
        @param seed - the seed for the perlin noise
        @param position - the center of the planet
        @param size - the radius of the planet
     */
    pub fn new(seed: u32, position: Vect2D, size: i16) -> Self {
        LandMass {
            seed: seed,
            position: position,
            size: size,
            rotationalAngle: 0.0,
        }
    }
    /*
        Update the LandMass rotation angle
     */
    pub fn update(&mut self) {
        self.rotationalAngle += 0.01;
    }
    /*
        draw the LandMass
        @param canvas - the canvas to draw to
        @param ColorScheme - defines the color scheme of the planet
     */
    pub fn draw(&mut self, canvas: &mut Canvas<Window>, ColorScheme: ColorScheme) {
        let xStart: i32 = self.position.x as i32 - self.size as i32;
        let yStart: i32 = self.position.y as i32 - self.size as i32;

        let perlin: Perlin = Perlin::new(self.seed);

        for xi in xStart .. xStart + 2*self.size as i32 {
            for yi in yStart.. yStart + 2*self.size as i32 {

                let dx: i32 = xi - self.position.x as i32;
                let dy: i32 = yi - self.position.y as i32;

                if ((dx*dx + dy*dy) as f64).sqrt() < self.size as f64 {

                    let coordinatesOnSphere: Vect2D =  PixelCoordinatesToSphereCoordinates(
                        Vect2D {x: xi as f64, y: yi as f64},
                        self.position,
                        2*self.size);

                    let perlinValue: f64 = 2.0*perlin.get([
                        coordinatesOnSphere.x+self.rotationalAngle, 
                        coordinatesOnSphere.y, 
                        0f64]);

                    let drawColor = match perlinValue {
                        x if x < 0.2 => ColorScheme.quaternary,
                        x if x < 0.4 => ColorScheme.tertiary,
                        x if x < 0.6 => ColorScheme.secondary,
                        _ => ColorScheme.primary,
                    };

                    canvas.set_draw_color(drawColor);
                    match canvas.draw_point(Point::new(xi, yi)) {
                        Ok(_) => (),
                        Err(_) => {}
                    };
                }
            }
        }
    }
}