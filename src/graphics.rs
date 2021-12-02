use crate::constvals::Consts;

pub struct Graphics {
    pixels: [[char; Consts::WINDOW_W]; Consts::WINDOW_H],
}

impl Graphics {
    pub fn new() -> Self {
        let gr = Graphics {
            pixels: [['e'; Consts::WINDOW_W]; Consts::WINDOW_H],
        };
        return gr;
    }

    pub fn debug(self) {
        for i in 0..self.pixels.len() {
            for j in 0..self.pixels[0].len() {
                print!("{}", self.pixels[i][j]);
            }
            println!();
        }
    }
}