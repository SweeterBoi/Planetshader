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

/*
    Struct representing the LandMass of a Planet with
        a center position
        a size defining the radius
        a seed for the perlin noise
        a rotationAngle for the animation
 */
pub struct Atmosphere{
    position: Vect2D,
    size: i16,
    seed: u32,
    rotationalAngle: f64,
}

impl Atmosphere {

    pub fn new(seed: u32, position: Vect2D, size: i16) -> Self {
        Atmosphere {
            seed: seed,
            position: position,
            size: (size as f64 * 1.01) as i16,
            rotationalAngle: 0.0,
        }
    }
    /*
        Update the LandMass rotation angle
     */
    pub fn update(&mut self) {
        self.rotationalAngle += 0.03;
    }
    /*
        draw the Atmosphere
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

                    let grid: f64 = 1.9;
                    let perlinValue = 2.0*perlin.get([
                        grid * coordinatesOnSphere.y, 
                        grid * (coordinatesOnSphere.x)+self.rotationalAngle, 
                        0f64]);

                    canvas.set_draw_color(Color::RGB(250, 250, 255));
                    if perlinValue > 0.9 {
                        match canvas.draw_point(Point::new(xi, yi)) {
                            Ok(_) => (),
                            Err(_) => {}
                        };
                    }
                }
            }
        }
    }
}