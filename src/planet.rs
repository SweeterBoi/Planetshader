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
use crate::landmass::LandMass;
use crate::atmoshphere::Atmosphere;

use std::f64::consts::PI;


/*
    Sttruct defines a Planet with
        a Landmass containing islands and oceans
        an Atmosphere with clouds
        a seed for the perlin noise
        a center position
        a size defining the radius
        a ColorScheme
 */
pub struct Planet{ 
    LandMass: LandMass,
    Atmosphere: Atmosphere,
    seed: u32,
    position: Vect2D,
    size: i16,
    ColorScheme: ColorScheme,
}

/*
    Implementation of the Planet struct
 */
impl Planet {
    /*
        Istantiates a new Planet
        @param ColorScheme - defines the color scheme of the planet

        @return a new Planet
     */
    pub fn new(ColorScheme: ColorScheme) -> Self {
        let seed = rand::thread_rng().gen();
        let position = Vect2D{x: (SCREENWIDTH as f64)/2f64, y: (SCREENHEIGHT as f64)/2f64};
        let size = 200i16;
        Planet {
            seed: seed,
            position: position,
            size: size,
            LandMass: LandMass::new(seed, position, size),
            Atmosphere: Atmosphere::new(seed, position, size),
            ColorScheme: ColorScheme,
            
        }
    }

    /*
        Update the Planet (e.g. Rotation)
     */
    pub fn update(&mut self) {
        self.LandMass.update();
        self.Atmosphere.update();
    }
    /*
        Draw wach component of the Planet
        @param canvas - the canvas to draw to
     */
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        self.LandMass.draw(canvas, self.ColorScheme);
        self.Atmosphere.draw(canvas, self.ColorScheme);
    }
}