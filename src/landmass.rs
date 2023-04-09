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
    /* THIS IS DEPRECATED
        Update the LandMass rotation angle
     */
    pub fn update(&mut self) {
        self.rotationalAngle += 0.01;
    }

    pub fn getPixelValue(&mut self, perlinLandmass: f64, ColorScheme: Vec<Color>, dx:i32, dy:i32, illuminationFunction: &dyn Fn(i32, i32, i32) -> bool) -> Color {
        let landMassDarkening = 40;

        let perlinColor = match perlinLandmass {
            x if x > 1.0 => ColorScheme[0],
            x if x > 0.6 => ColorScheme[1],
            x if x > 0.4 => ColorScheme[2],
            x if x > 0.3 => ColorScheme[3],
            _ => ColorScheme[4]
        };

        // Check wether the pixel value has to be darkened due to missing illumination
        let isNotIllunimated: bool = illuminationFunction(dx, dy, self.size.into());
        let drawColor: Color;
        if isNotIllunimated {
        drawColor =  Color::RGB(
            perlinColor.r-landMassDarkening, 
            perlinColor.g-landMassDarkening, 
            perlinColor.b-landMassDarkening);
        }
        else {
        drawColor = perlinColor;
        }
        drawColor
    }
}