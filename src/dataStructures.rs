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


pub struct ColorScheme {
    primary: Color,
    secondary: Color,
    tertiary: Color,
    quaternary: Color
}

impl ColorScheme {

    pub fn new() -> ColorScheme {
        let mut cols: Vec<Color> = Vec::new();
        for _ in 0..4 {
            let r: u8 = rand::thread_rng().gen_range(0..255);
            let g: u8 = rand::thread_rng().gen_range(0..255);
            let b: u8 = rand::thread_rng().gen_range(0..255);
            cols.push(Color::RGB(r, g, b));
        }
        ColorScheme { primary: cols[0], secondary: cols[1], tertiary: cols[2], quaternary: cols[3] }
    }
    
}
