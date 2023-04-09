use rand::Rng;
use sdl2::pixels::Color;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vect2D {
    pub x: f64,
    pub y: f64
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vect3D {
    x: f64,
    y: f64,
    z: f64
}

impl From<Vect2D> for Vect3D {
    fn from(vec: Vect2D) -> Self {
        Vect3D{x: vec.x, y: vec.y, z: 0f64}
    }
}

impl From<Vect3D> for Vect2D {
    fn from(vec: Vect3D) -> Self {
        Vect2D{x: vec.x, y: vec.y}
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ColorScheme {
    pub c0: Color,
    pub c1: Color,
    pub c2: Color,
    pub c3: Color,
    pub c4: Color,
    pub c5: Color,
    pub c6: Color,
    pub c7: Color,
    pub c8: Color,
    pub c9: Color,
}

impl ColorScheme {

    pub fn new() -> ColorScheme {
        let mut cols: Vec<Color> = Vec::new();
        for _ in 0..10 {
            let r: u8 = rand::thread_rng().gen_range(0..255);
            let g: u8 = rand::thread_rng().gen_range(0..255);
            let b: u8 = rand::thread_rng().gen_range(0..255);
            cols.push(Color::RGB(r, g, b));
        }
        ColorScheme { 
            c0: cols[0],
            c1: cols[1],
            c2: cols[2],
            c3: cols[3],
            c4: cols[4],
            c5: cols[5],
            c6: cols[6],
            c7: cols[7],
            c8: cols[8],
            c9: cols[9]
        }
    }
    
}
