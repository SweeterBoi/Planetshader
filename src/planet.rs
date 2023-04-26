use rand::Rng;
use sdl2::sys::ScreenSaverActive;
use sdl2::{pixels::Color, gfx::primitives::DrawRenderer};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::{Rect, Point};
use std::time::{Duration, Instant};

use noise::{Perlin, NoiseFn, Seedable};

use crate::{SCREENWIDTH, SCREENHEIGHT};
use crate::dataStructures::{Vect2D, ColorScheme};
use crate::converter::converter::PixelCoordinatesToSphereCoordinates;
use crate::landmass::LandMass;
use crate::atmoshphere::Atmosphere;

use std::f64::consts::PI;

 
const landMassGrid: f64 = 1.0;
const atmosphereGrid: f64 = 2.0;


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
    ColorScheme: Vec<Color>,
    rotationalAngle: f64,
    coordinateTable: Vec<Vec<Vect2D>>,
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
    
    pub fn new(ColorScheme: Vec<Color>, size: i16) -> Self {
        let seed = rand::thread_rng().gen();
        let position = Vect2D{x: (SCREENWIDTH as f64)/2f64, y: (SCREENHEIGHT as f64)/2f64};
        Planet {
            seed: seed,
            position: position,
            size: size,
            LandMass: LandMass::new(seed, position, size),
            Atmosphere: Atmosphere::new(seed, position, size),
            ColorScheme: ColorScheme,
            rotationalAngle: 0f64,
            coordinateTable: Self::generateCoordinatTable(position, size),
        }
    }

    /*
        Update the Planet (e.g. Rotation)
     */
    pub fn update(&mut self) {
        self.rotationalAngle += 0.01;
        //self.LandMass.update();
        //self.Atmosphere.update();
    }
    /*
        Draw wach component of the Planet
        @param canvas - the canvas to draw to
     */
    fn isIlluminated(x: i32, y:i32, size: i32) -> bool {
        let xOffset = 30;
        let yOffset = 20;
        let radius: i32 = (1.0 * size as f64) as i32;
        (x + xOffset).pow(2) + (y + yOffset).pow(2) > radius.pow(2)
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        
        let xStart: i32 = self.position.x as i32 - self.size as i32;
        let yStart: i32 = self.position.y as i32 - self.size as i32;
        
        let perlin: Perlin = Perlin::new(self.seed);
        
        for xi in xStart .. xStart + 2*self.size as i32 {
            for yi in yStart.. yStart + 2*self.size as i32 {

                let dx: i32 = xi - self.position.x as i32;
                let dy: i32 = yi - self.position.y as i32;

                if ((dx*dx + dy*dy) as f64).sqrt() >= self.size as f64 {
                    continue;
                }
                // get pixelcoordinates as coordinates on the sphere
                let coordinatesOnSphere: Vect2D =  self.coordinateTable[(xi - xStart) as usize][(yi -yStart) as usize];
                // get perlin value for the landmass
                let perlinLandmass: f64 = 8.0*perlin.get([
                    landMassGrid * coordinatesOnSphere.x+self.rotationalAngle, 
                    landMassGrid * coordinatesOnSphere.y, 
                    0f64]);
                // get perlin value for the atmosphere
                let perlinAtmosphere = 8.0*perlin.get([
                    atmosphereGrid * coordinatesOnSphere.y, 
                    atmosphereGrid * coordinatesOnSphere.x+4.0*self.rotationalAngle, 
                    3.0*self.rotationalAngle]);
                // draw atmosphere over the landmass
                if perlinAtmosphere > 0.8 {
                    let drawColor = self.Atmosphere.getPixelValue(perlinAtmosphere, self.ColorScheme.clone(), dx, dy, &Self::isIlluminated);
                    canvas.set_draw_color(drawColor);
                    }
                // draw landmass instead
                else {
                    let drawColor = self.LandMass.getPixelValue(perlinLandmass, self.ColorScheme.clone(), dx, dy, &Self::isIlluminated);
                    // Set Color
                    canvas.set_draw_color(drawColor);
                }
                match canvas.draw_point(Point::new(xi, yi)) {
                    Ok(_) => (),
                    Err(_) => {}
                    };
            }
        }
    }

    fn generateCoordinatTable(position: Vect2D, size: i16) -> Vec<Vec<Vect2D>> {
        let mut output: Vec<Vec<Vect2D>> = Vec::new(); 
        let xStart: i32 = position.x as i32 - size as i32;
        let yStart: i32 = position.y as i32 - size as i32;
        
        for xi in xStart .. xStart + 2*size as i32 {
            let mut row: Vec<Vect2D> = Vec::new(); 
            for yi in yStart.. yStart + 2*size as i32 {
                let coords: Vect2D = PixelCoordinatesToSphereCoordinates(Vect2D {x: xi as f64, y: yi as f64}, position, size);
                row.push(coords);
            }
        output.push(row);
        }
        output
    }
}
