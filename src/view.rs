use crate::constvals::Consts;
use crate::model::Model;
use colored::*;

pub struct View {
    pixels: [[char; Consts::WINDOW_W]; Consts::WINDOW_H],
}

impl View {
    pub fn new() -> Self {
        let view = View {
            pixels: [[' '; Consts::WINDOW_W]; Consts::WINDOW_H],
        };
        return view;
    }

    pub fn update(&mut self, model: &Model) {
        self.pixels = model.get_map();
    }

    pub fn draw(&self) {
        for i in 0..self.pixels.len() {
            let mut s = "".to_string();
            for j in 0..self.pixels[0].len() {
                s.push(self.pixels[i][j]);
            }
            println!("{}", s.green());
        }
    }

    pub fn clean(&self) {
        print!("{}", Consts::clean());
    }
}

