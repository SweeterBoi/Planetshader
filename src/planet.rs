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
            Atmosphere: Atmosphere::new(),
            ColorScheme: ColorScheme,
            
        }
    }

    /*
        Update the Planet (e.g. Rotation)
     */
    pub fn update(&mut self) {
        self.LandMass.update();
    }
    /*
        Draw wach component of the Planet
        @param canvas - the canvas to draw to
     */
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        self.LandMass.draw(canvas, self.ColorScheme);
    }
}

/*
    Struct representing the LandMass of a Planet with
        a center position
        a size defining the radius
        a seed for the perlin noise
        a rotationAngle for the animation
 */
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

                    let perlinValue = 2.0*perlin.get([
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

pub struct Atmosphere{}

impl Atmosphere {
    pub fn new() -> Self {
        Atmosphere {}
    }
}