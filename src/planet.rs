use rand::Rng;
use sdl2::{pixels::Color, gfx::primitives::DrawRenderer};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::{Rect, Point};

use noise::{Perlin, NoiseFn, Seedable};

use crate::{SCREENWIDTH, SCREENHEIGHT};
use crate::dataStructures::Vect2D;
use crate::converter::converter::PixelCoordinatesToSphereCoordinates;
use std::f64::consts::PI;


pub struct Planet{ 
    LandMass: LandMass,
    Atmosphere: Atmosphere,
    seed: u32,
    position: Vect2D,
    size: i16
}

impl Planet {
    pub fn new() -> Self {
        let seed = rand::thread_rng().gen();
        let position = Vect2D{x: (SCREENWIDTH as f64)/2f64, y: (SCREENHEIGHT as f64)/2f64};
        let size = 200i16;
        Planet {
            seed: seed,
            position: position,
            size: size,
            LandMass: LandMass::new(seed, position, size),
            Atmosphere: Atmosphere::new(),
            
        }
    }
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        match canvas.circle(self.position.x as i16, self.position.y as i16, self.size, Color::RGB(255, 0, 0)) {
            Ok(_) => {}
            Err(_) => {}
        }
        self.LandMass.draw(canvas);
    }
}


pub struct LandMass {
    position: Vect2D,
    size: i16,
    seed: u32
}

impl LandMass {
    pub fn new(seed: u32, position: Vect2D, size: i16) -> Self {
        LandMass {
            seed: seed,
            position: position,
            size: size,
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let xStart: i32 = self.position.x as i32 - self.size as i32 /2;
        let yStart: i32 = self.position.y as i32 - self.size as i32 /2;

        let perlin = Perlin::new(self.seed);

        for xi in xStart .. xStart + self.size as i32 {
            for yi in yStart.. yStart + self.size as i32 {
                if ((xi*xi + yi*yi) as f64).sqrt() <= self.size as f64 {

                    let coordinatesOnSphere: Vect2D =  PixelCoordinatesToSphereCoordinates(
                        Vect2D {x: xi as f64, y: yi as f64},
                        self.position,
                        self.size);

                    let perlinValue = perlin.get([coordinatesOnSphere.x, coordinatesOnSphere.y, 0f64]);

                    let mut drawColor = Color::RGB(0, 0, 0);

                    if perlinValue < 0.5 {
                        drawColor = Color::RGB(255, 0, 0);
                    } else {
                        drawColor = Color::RGB(0, 255, 0);
                    }

                    canvas.set_draw_color(drawColor);
                    match canvas.draw_point(Point::new(xStart + xi,yStart + yi)) {
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