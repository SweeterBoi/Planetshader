use rand::Rng;
use sdl2::{pixels::Color, gfx::primitives::DrawRenderer};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

use crate::{SCREENWIDTH, SCREENHEIGHT};
use crate::dataStructures::Vect2D;

pub struct BackgroundStars {
    stars: Vec<DotStar>
}

impl BackgroundStars {
    pub fn new(num: u32) -> BackgroundStars {
        let mut tempStarsList: Vec<DotStar> = Vec::new();
        for _ in 0 .. num {
            tempStarsList.push(DotStar::new());
        }
        BackgroundStars { stars: tempStarsList }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) -> () { 
        for star in &mut self.stars {
            star.update();
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            match star.draw(canvas) {
                Ok(_) => (),
                Err(_) => (),
            };
        }
    }
}

pub struct DotStar {
    position: Vect2D,
    isBright: bool
}

impl DotStar {
    fn new() -> DotStar {
        let mut rng = rand::thread_rng();
        let pos = Vect2D { x: rng.gen_range(0.0 .. SCREENWIDTH as f64), y: rng.gen_range(0.0 .. SCREENHEIGHT as f64)};
        DotStar { position: pos, isBright: false }
    }

    fn update(&mut self) {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0 .. 100_000);
        if num == 0 {
            self.isBright = true;
        }
        else if num <= 30_000 {
            self.isBright = false;
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let size: u32 = match self.isBright {
            true => 4,
            false => 2
        };
        canvas.fill_rect(Rect::new(self.position.x as i32, self.position.y as i32, size, size))
    }
}