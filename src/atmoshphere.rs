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
    /* THIS IS DEPRECATED
        Update the LandMass rotation angle
     */
    pub fn update(&mut self) {
        self.rotationalAngle += 0.03;
    }

    /*
        
    */
    pub fn getPixelValue(&mut self, perlinAtmosphere: f64, ColorScheme: Vec<Color>, dx:i32, dy:i32, illuminationFunction: &dyn Fn(i32, i32, i32) -> bool) -> Color {
        let cloudDarkening = 30;
        
        let cloudColor = match perlinAtmosphere {
            x if x > 1.4 => ColorScheme[6],
            _=> ColorScheme[7],
        };

        // Darken the Clouds where they are not illuminated
        let isNotIllunimated: bool = illuminationFunction(dx, dy, self.size.into());
        let drawColor: Color;
        if isNotIllunimated {
        drawColor =  Color::RGB(
            cloudColor.r-cloudDarkening, 
            cloudColor.g-cloudDarkening, 
            cloudColor.b-cloudDarkening);
        }
        else {
        drawColor = cloudColor;
        }
        drawColor
    }
}